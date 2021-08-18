use ultraviolet::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3, 
    pub direction: Vec3
    pub tm: f64
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}