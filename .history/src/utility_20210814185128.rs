use ultraviolet::*;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { 
        return min;
    };
    if x > max {
        return max;
    };
    return max;
}

pub fn random_vec3() -> Vec3<f32> {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}