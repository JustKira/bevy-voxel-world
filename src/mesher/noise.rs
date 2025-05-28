use fastnoise_lite::FastNoiseLite;

pub struct NoiseMesher {
    fast_noise: FastNoiseLite,
}

impl NoiseMesher {
    pub fn new(fast_noise: FastNoiseLite) -> Self {
        NoiseMesher { fast_noise }
    }

    pub fn get_noise_clamped_height(
        &self,
        min_height: f32,
        max_height: f32,
        x: f32,
        y: f32,
    ) -> f32 {
        self.fast_noise.domain_warp_2d(x, y);

        ((self.fast_noise.get_noise_2d(x, y) + 1.0) / 2.0) * (max_height - min_height) + min_height
    }

    // return noise between -1 and 1
    pub fn get_noise_3d(&self, x: f32, y: f32, z: f32) -> f32 {
        self.fast_noise.domain_warp_3d(x, y, z);

        self.fast_noise.get_noise_3d(x, y, z)
    }

    // return noise between -1 and 1
    pub fn get_noise_2d(&self, x: f32, y: f32) -> f32 {
        self.fast_noise.domain_warp_2d(x, y);
        self.fast_noise.get_noise_2d(x, y)
    }

    // return noise between 0 and 1
    pub fn get_noise_bounded_3d(&self, x: f32, y: f32, z: f32) -> f32 {
        self.fast_noise.get_noise_3d(x, y, z)
    }

    pub fn get_noise_bounded_2d(&self, x: f32, y: f32) -> f32 {
        let noise = self.fast_noise.get_noise_2d(x, y);
        noise.clamp(-1.0, 1.0)
    }
}
