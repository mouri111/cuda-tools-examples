use crate::vec3::*;

#[repr(C)]
#[derive(Clone,Copy,Default)]
pub struct Ray {
    pub a: Vec3,
    pub d: Vec3,
}

impl Ray {
    #[inline(always)]
    pub fn new() -> Ray {
        Ray {
            a: Vec3::new(),
            d: Vec3::new(),
        }
    }

    #[inline(always)]
    pub fn new_from_origin_and_direction(a: Vec3, d: Vec3) -> Ray {
        Ray { a, d }
    }

    #[inline(always)]
    pub fn origin(&self) -> Vec3 {
        self.a
    }

    #[inline(always)]
    pub fn direction(&self) -> Vec3 {
        self.d
    }

    #[inline(always)]
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t * self.d
    }
}
