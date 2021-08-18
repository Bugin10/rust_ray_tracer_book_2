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

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    while p.mag_sq() >= 1.0 {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
             * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

pub fn random_unit_vector() -> Vec3 {
    return (random_in_unit_sphere()).normalized();
}


pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.mag_sq() >= 1.0 {continue;}
        return p;
    }
}

pub fn near_zero(&Vec3 vec) -> bool {
    let s = 1e-8;
    return self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
}