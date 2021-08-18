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

pub fn random_unit_vector() -> Vec3 {
    return unit_vector(random_in_unit_sphere());
}


pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.mag_sq() >= 1.0 {continue;}
        return p;
    }
}