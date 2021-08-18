use ultraviolet::*;
use rand::Rng;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { 
        return min;
    };
    if x > max {
        return max;
    };
    return max;
}


// Vector related utilities

pub fn random_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}


