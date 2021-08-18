use ultraviolet::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3, 
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        Ray { origin, direction, time }
    }

    pub fn at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}