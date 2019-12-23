use crate::vec3::*;
use crate::ray::*;
use crate::hit_record::*;
use crate::xorshift::*;

#[derive(Clone,Copy)]
pub struct Object {
    pub shape: ObjectShape,
    pub material: ObjectMaterial,
}

#[derive(Clone,Copy)]
pub enum ObjectShape {
    Sphere {
        center: Vec3,
        radius: f32,
    }
}

#[derive(Clone,Copy)]
pub enum ObjectMaterial {
    Lambertian {
        albedo: Vec3,
    },
    Metal {
        albedo: Vec3,
        fuzz: f32,
    },
    Dielectric {
        ref_idx: f32,
    }
}

#[inline(always)]
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    let t = 1.0 - cosine;
    r0 + (1.0 - r0) * t * t * t * t * t
}

impl ObjectMaterial {
    #[inline(always)]
    pub fn scatter(&self, xorshift: &mut XorShift, ray_in: Ray, hit_record: HitRecord) -> Option<(Vec3,Ray)> {
        match *self {
            ObjectMaterial::Lambertian{albedo} => {
                let target = hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere(xorshift);
                let scattered = Ray::new_from_origin_and_direction(hit_record.p, target - hit_record.p);
                let attenuation = albedo;
                Some((attenuation, scattered))
            }
            ObjectMaterial::Metal{albedo,fuzz} => {
                let reflected = Vec3::reflect(Vec3::unit_vector(ray_in.direction()), hit_record.normal);
                let scattered = Ray::new_from_origin_and_direction(hit_record.p, reflected + fuzz * Vec3::random_in_unit_sphere(xorshift));
                let attenuation = albedo;
                if Vec3::dot(scattered.direction(), hit_record.normal) > 0.0 {
                    Some((attenuation,scattered))
                }
                else {
                    None
                }
            }
            ObjectMaterial::Dielectric{ref_idx} => {
                let reflected = Vec3::reflect(ray_in.direction(), hit_record.normal);
                let attenuation = Vec3{x: 1.0, y: 1.0, z: 1.0};
                let outward_normal;
                let ni_over_nt;
                let cosine;
                if Vec3::dot(ray_in.direction(), hit_record.normal) > 0.0 {
                    outward_normal = -hit_record.normal;
                    ni_over_nt = ref_idx;
                    cosine = ref_idx * Vec3::dot(ray_in.direction(), hit_record.normal) / ray_in.direction().length();
                }
                else {
                    outward_normal = hit_record.normal;
                    ni_over_nt = 1.0 / ref_idx;
                    cosine = -Vec3::dot(ray_in.direction(), hit_record.normal) / ray_in.direction().length();
                }
                if let Some(refracted) = Vec3::refract(ray_in.direction(), outward_normal, ni_over_nt) {
                    let reflect_prob = schlick(cosine, ref_idx);
                    let scattered = if xorshift.gen_f32() < reflect_prob {
                        Ray::new_from_origin_and_direction(hit_record.p, reflected)
                    }
                    else {
                        Ray::new_from_origin_and_direction(hit_record.p, refracted)
                    };
                    Some((attenuation,scattered))
                }
                else {
                    let scattered = Ray::new_from_origin_and_direction(hit_record.p, reflected);
                    Some((attenuation,scattered))
                }
            }
        }
    }
}

impl Object {
    #[inline(always)]
    pub fn hit(&self, object_id: usize, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.shape {
            ObjectShape::Sphere{center,radius} => {
                let oc = ray.origin() - center;
                let a = Vec3::dot(ray.direction(), ray.direction());
                let b = Vec3::dot(oc, ray.direction());
                let c = Vec3::dot(oc, oc) - radius * radius;
                let discriminant = b * b - a * c;
                if discriminant > 0.0 {
                    use core::intrinsics::sqrtf32;
                    let t = unsafe { (-b - sqrtf32(discriminant)) / a };
                    if t_min < t && t < t_max {
                        let p = ray.point_at_parameter(t);
                        let normal = (p - center) / radius;
                        return Some(HitRecord{t,p,normal,object_id});
                    }
                    let t = unsafe { (-b + sqrtf32(discriminant)) / a };
                    if t_min < t && t < t_max {
                        let p = ray.point_at_parameter(t);
                        let normal = (p - center) / radius;
                        return Some(HitRecord{t,p,normal,object_id});
                    }
                }
                None
            }
        }
    }
}
