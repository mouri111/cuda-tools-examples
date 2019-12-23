use core::ops::*;
use crate::xorshift::*;

#[repr(C)]
#[derive(Clone,Copy,Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, t: f32) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, t: f32) {
        *self = Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        };
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, t: f32) -> Self {
        Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, t: f32) {
        *self = Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        };
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn new() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline(always)]
    pub fn sqrt(&self) -> Vec3 {
        use core::intrinsics::sqrtf32;
        Vec3 {
            x: unsafe { sqrtf32(self.x) },
            y: unsafe { sqrtf32(self.y) },
            z: unsafe { sqrtf32(self.z) },
        }
    }

    #[inline(always)]
    pub fn powf(&self, a: f32) -> Vec3 {
        use core::intrinsics::powf32;
        Vec3 {
            x: unsafe { powf32(self.x, a) },
            y: unsafe { powf32(self.y, a) },
            z: unsafe { powf32(self.z, a) },
        }
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        use core::intrinsics::sqrtf32;
        unsafe {
            sqrtf32(self.x * self.x + self.y * self.y + self.z * self.z)
        }
    }

    #[inline(always)]
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline(always)]
    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    #[inline(always)]
    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        v1.x * v2.x + v1.y * v2.y  + v1.z * v2.z
    }

    #[inline(always)]
    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: -(v1.x * v2.z - v1.z * v2.x),
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    #[inline(always)]
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v,n) * n
    }

    #[inline(always)]
    pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = Vec3::unit_vector(v);
        let dt = Vec3::dot(uv, n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            use core::intrinsics::sqrtf32;
            let refracted = ni_over_nt * (uv - n * dt) - n * unsafe{sqrtf32(discriminant)};
            Some(refracted)
        }
        else {
            None
        }
    }

    #[inline(always)]
    pub fn random_in_unit_sphere(xorshift: &mut XorShift) -> Vec3 {
        loop {
            let p = 2.0 * Vec3{x: xorshift.gen_f32(), y: xorshift.gen_f32(), z: xorshift.gen_f32()} - Vec3{x:1.0, y: 1.0, z: 1.0};
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    pub fn i(&self, i: usize) -> f32 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }
}
