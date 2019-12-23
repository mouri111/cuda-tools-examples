pub struct XorShift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorShift {
    #[inline(always)]
    pub fn new(seed: u32) -> XorShift {
        XorShift {
            x: 123_456_789,
            y: 362_436_069,
            z: 521_288_629,
            w: seed,
        }
    }

    #[inline(always)]
    pub fn gen_u32(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let res = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        self.w = res;
        res
    }

    #[inline(always)]
    pub fn gen_f32(&mut self) -> f32 {
        self.gen_u32() as f32 / 4_294_967_296i64 as f32
    }
}
