use crate::vec3::*;
use crate::object::*;
use crate::camera::*;
use cuda_tools::cuda_slice::*;
use core::cell::UnsafeCell;

pub struct RayTraceArgs<'a> {
    pub image_len: usize,
    pub image: CUDASlice<'a, UnsafeCell<Vec3>>,
    pub h: usize,
    pub w: usize,
    pub objects_len: usize,
    pub objects: CUDASlice<'a, Object>,
    pub ray_per_pixel: usize,
    pub camera: Camera,
}
