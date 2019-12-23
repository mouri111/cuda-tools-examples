#[cfg(target_arch = "nvptx64")]
use crate::ray::*;
#[cfg(target_arch = "nvptx64")]
use crate::vec3::*;
use crate::ray_trace_args::*;
#[cfg(target_arch = "nvptx64")]
use crate::hit_record::*;
#[cfg(target_arch = "nvptx64")]
use crate::xorshift::*;

#[cfg(target_arch = "nvptx64")]
fn color(args: &RayTraceArgs, xorshift: &mut XorShift, ray: Ray) -> Vec3 {
    let mut ratio = Vec3{x: 1.0, y: 1.0, z: 1.0};
    let mut ray = ray;
    for _ in 0..50 {
        if let Some(rec) = hit(args, ray, 0.001, 1e10) {
            if rec.object_id < args.objects.len() {
                if let Some((attenuation, scattered)) = args.objects[rec.object_id].material.scatter(xorshift, ray, rec) {
                    ratio *= attenuation;
                    ray = scattered;
                }
                else {
                    return Vec3{x: 0.0, y: 0.0, z: 0.0}
                }                
            }
        }
        else {
            let unit_direction = Vec3::unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y + 1.0);
            let s = (1.0 - t) * Vec3 {x: 1.0, y: 1.0, z: 1.0} + t * Vec3 {x: 0.5, y: 0.7, z: 1.0};
            return ratio * s;
        }
    }
    Vec3{x: 0.0, y: 0.0, z: 0.0}
}

#[cfg(target_arch = "nvptx64")]
fn hit(args: &RayTraceArgs, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut res = None;
    let mut closest_so_far = t_max;
    for i in 0..args.objects_len {
        if i < args.objects.len() {
            if let Some(rec) = args.objects[i].hit(i, ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                res = Some(rec);
            }
        }
    }
    res
}

#[cfg(target_arch = "nvptx64")]
unsafe fn atomic_add_f32(ptr: *mut f32, x: f32) {
    loop {
        let old1 = *ptr;
        let old = old1.to_bits();
        let new = (old1 + x).to_bits();
        let (res, _) = core::intrinsics::atomic_cxchg::<u32>(ptr as *mut u32, old, new);
        if res == old {
            break;
        }
    }
}

#[no_mangle]
#[cfg(not(target_arch = "nvptx64"))]
pub extern "ptx-kernel" fn ray_trace(args: &RayTraceArgs) {}

#[no_mangle]
#[cfg(target_arch = "nvptx64")]
pub extern "ptx-kernel" fn ray_trace(args: &RayTraceArgs) {
    let image_len = args.image_len;
    let h = args.h;
    let w = args.w;
    let i = unsafe { core::arch::nvptx::_block_idx_x() * core::arch::nvptx::_block_dim_x() + core::arch::nvptx::_thread_idx_x() } as isize;
    let ray_per_pixel = args.ray_per_pixel;
    if h != 0 && w != 0 && ray_per_pixel != 0 {
        let x = i as usize / ray_per_pixel % w;
        let y = h - i as usize / ray_per_pixel / w - 1;
        let seed = i as u32;
        let mut xorshift = XorShift::new(seed);
        for _ in 0..i as usize % ray_per_pixel {
            xorshift.gen_u32();
        }
        let mut res = Vec3{x:0.0, y: 0.0, z: 0.0};
        let camera = args.camera;
        let u = (x as f32 + xorshift.gen_f32()) / w as f32;
        let v = (y as f32 + xorshift.gen_f32()) / h as f32;
        let ray = camera.get_ray(&mut xorshift, u, v);
        let col = color(args, &mut xorshift, ray);
        res += col;
        res = res.sqrt();
        res /= ray_per_pixel as f32;

        let vec3 = Vec3::new();
        let x_offset = &vec3.x as *const f32 as usize - &vec3 as *const Vec3 as usize;
        let y_offset = &vec3.y as *const f32 as usize - &vec3 as *const Vec3 as usize;
        let z_offset = &vec3.z as *const f32 as usize - &vec3 as *const Vec3 as usize;

        unsafe {
            let i = i as usize / ray_per_pixel;
            if i < args.image.len() {
                let p = args.image[i].get();
                atomic_add_f32((p as usize + x_offset) as *mut f32, res.x);
                atomic_add_f32((p as usize + y_offset) as *mut f32, res.y);
                atomic_add_f32((p as usize + z_offset) as *mut f32, res.z);
            }
        }
    }
}
