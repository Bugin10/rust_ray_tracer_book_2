use ultraviolet::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3x8, 
    pub direction: Vec3x8
}

impl Ray {
    pub fn new(origin: Vec3x8, direction: Vec3x8) -> Ray {
        Ray { origin, direction }
    }

    pub fn at_parameter(&self, t: f32) -> Vec3x8 {
        self.origin + t * self.direction
    }
}