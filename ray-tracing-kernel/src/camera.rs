use crate::vec3::*;
use crate::ray::*;
use crate::xorshift::*;

#[derive(Clone,Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

#[inline(always)]
fn random_in_unit_disk(xorshift: &mut XorShift) -> Vec3 {
    loop {
        let p = 2.0 * Vec3{x: xorshift.gen_f32(), y: xorshift.gen_f32(), z: 0.0} - Vec3{x: 1.0, y: 1.0, z: 0.0};
        if Vec3::dot(p,p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    #[inline(always)]
    pub fn get_ray(&self, xorshift: &mut XorShift, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(xorshift);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new_from_origin_and_direction(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}