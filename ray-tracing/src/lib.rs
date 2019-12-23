#![feature(abi_ptx)]

#[cfg(not(target_arch = "nvptx64"))]
#[macro_use]
extern crate cuda_tools;

use core::cell::UnsafeCell;
use ray_tracing_kernel as kernel;

const KERNEL: &str = include_kernel!();

use kernel::camera::*;
use kernel::object::*;
use kernel::ray_trace_args::*;
use kernel::vec3::*;

pub fn new_camera(
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f32,
    aspect: f32,
    aperture: f32,
    focus_dist: f32,
) -> Camera {
    let lens_radius = aperture / 2.0;
    let theta = vfov * 3.141592653589793238f32 / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let origin = lookfrom;
    let w = Vec3::unit_vector(lookfrom - lookat);
    let u = Vec3::unit_vector(Vec3::cross(vup, w));
    let v = Vec3::cross(w, u);
    let lower_left_corner =
        origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
    let horizontal = 2.0 * half_width * focus_dist * u;
    let vertical = 2.0 * half_height * focus_dist * v;
    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
        u,
        v,
        w,
        lens_radius,
    }
}

fn small_scene(seed: u32) -> Vec<Object> {
    let objects = vec![
        Object {
            shape: ObjectShape::Sphere {
                center: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            },
            material: ObjectMaterial::Lambertian {
                albedo: Vec3 {
                    x: 0.1,
                    y: 0.2,
                    z: 0.5,
                },
            },
        },
        Object {
            shape: ObjectShape::Sphere {
                center: Vec3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                radius: 100.0,
            },
            material: ObjectMaterial::Lambertian {
                albedo: Vec3 {
                    x: 0.8,
                    y: 0.8,
                    z: 0.0,
                },
            },
        },
        Object {
            shape: ObjectShape::Sphere {
                center: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            },
            material: ObjectMaterial::Metal {
                albedo: Vec3 {
                    x: 0.8,
                    y: 0.6,
                    z: 0.2,
                },
                fuzz: 0.0,
            },
        },
        Object {
            shape: ObjectShape::Sphere {
                center: Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            },
            material: ObjectMaterial::Dielectric { ref_idx: 1.5 },
        },
        Object {
            shape: ObjectShape::Sphere {
                center: Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: -0.45,
            },
            material: ObjectMaterial::Dielectric { ref_idx: 1.5 },
        },
    ];
    objects
}

fn random_scene(seed: u32) -> Vec<Object> {
    use kernel::xorshift::*;
    let mut xorshift = XorShift::new(seed);
    let mut res = vec![];
    res.push(Object {
        shape: ObjectShape::Sphere {
            center: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            radius: 1000.0,
        },
        material: ObjectMaterial::Lambertian {
            albedo: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        },
    });
    let size = 11;
    for a in -size..size {
        for b in -size..size {
            let choose_mat = xorshift.gen_f32();
            let center = Vec3 {
                x: a as f32 + 0.9 * xorshift.gen_f32(),
                y: 0.2,
                z: b as f32 + 0.9 * xorshift.gen_f32(),
            };
            if (center
                - Vec3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .length()
                > 0.9
            {
                if choose_mat < 0.8 {
                    res.push(Object {
                        shape: ObjectShape::Sphere {
                            center,
                            radius: 0.2,
                        },
                        material: ObjectMaterial::Lambertian {
                            albedo: Vec3 {
                                x: xorshift.gen_f32() * xorshift.gen_f32(),
                                y: xorshift.gen_f32() * xorshift.gen_f32(),
                                z: xorshift.gen_f32() * xorshift.gen_f32(),
                            },
                        },
                    });
                } else if choose_mat < 0.95 {
                    res.push(Object {
                        shape: ObjectShape::Sphere {
                            center,
                            radius: 0.2,
                        },
                        material: ObjectMaterial::Metal {
                            albedo: Vec3 {
                                x: 0.5 * (1.0 + xorshift.gen_f32()),
                                y: 0.5 * (1.0 + xorshift.gen_f32()),
                                z: 0.5 * (1.0 + xorshift.gen_f32()),
                            },
                            fuzz: 0.5 * xorshift.gen_f32(),
                        },
                    });
                } else {
                    res.push(Object {
                        shape: ObjectShape::Sphere {
                            center,
                            radius: 0.2,
                        },
                        material: ObjectMaterial::Dielectric { ref_idx: 1.5 },
                    });
                }
            }
        }
    }
    res.push(Object {
        shape: ObjectShape::Sphere {
            center: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
        },
        material: ObjectMaterial::Dielectric { ref_idx: 1.5 },
    });
    res.push(Object {
        shape: ObjectShape::Sphere {
            center: Vec3 {
                x: -4.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
        },
        material: ObjectMaterial::Lambertian {
            albedo: Vec3 {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            },
        },
    });
    res.push(Object {
        shape: ObjectShape::Sphere {
            center: Vec3 {
                x: 4.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
        },
        material: ObjectMaterial::Metal {
            albedo: Vec3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        },
    });
    res
}

pub fn run(seed: u32, height: usize, width: usize, ray_per_pixel: usize) {
    let mut runtime = cuda_tools::runtime::Runtime::new(0, KERNEL).unwrap();
    runtime.record_function_name(kernel::kernel::ray_trace, "ray_trace");

    let h = height;
    let w = width;
    let n_thread = h * w * ray_per_pixel;
    let n = h * w;

    let mut image_h = vec![];
    for _ in 0..n {
        image_h.push(UnsafeCell::new(Vec3::new()));
    }
    let image_d = runtime.alloc_slice(&image_h).unwrap();

    let m = 64;
    let objects = random_scene(seed);
    eprintln!("objects.len() = {}", objects.len());

    let objects_d = runtime.alloc_slice(&objects).unwrap();

    let lookfrom = Vec3 {
        x: 10.0,
        y: 2.0,
        z: 2.5,
    };
    let lookat = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let dist_to_focus: f32 = (lookfrom - lookat).length();
    let aperture = 0.00;

    let args = RayTraceArgs {
        image_len: n,
        image: image_d,
        h,
        w,
        objects_len: objects.len(),
        objects: objects_d,
        ray_per_pixel,
        camera: new_camera(
            lookfrom,
            lookat,
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            30.0,
            w as f32 / h as f32,
            aperture,
            dist_to_focus,
        ),
    };

    runtime
        .launch(
            kernel::kernel::ray_trace,
            &args,
            (n_thread + m - 1) / m,
            1,
            1,
            m,
            1,
            1,
        )
        .unwrap();

    let image = args.image.to_host().unwrap();
    let image: Vec<Vec3> = image.into_iter().map(|x| x.into_inner()).collect();

    println!("P3");
    println!("{} {}", w, h);
    println!("255");
    for y in 0..h {
        for x in 0..w {
            let v = image[y * w + x];
            let r = (255.99 * v.x) as i32;
            let g = (255.99 * v.y) as i32;
            let b = (255.99 * v.z) as i32;
            println!("{} {} {}", r, g, b);
        }
    }
}
