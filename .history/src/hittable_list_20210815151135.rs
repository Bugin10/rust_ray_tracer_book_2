use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::aabb::*;
use ultraviolet::*;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let mut closest_so_far = std::f32::INFINITY;
        let mut result = None;

        for object in self.objects.iter() {
            if let Some((hit_rec, material)) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_rec.t;
                result = Some((
                    HitRecord {
                        t: hit_rec.t,
                        p: hit_rec.p,
                        normal: hit_rec.normal,
                    },
                    material,
                ))
            }
        }
        result
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box;
        let mut first_box = true;
        for object in self.objects.iter() {
            if (!object.bounding_box(time0, time1, temp_box)) {
                return false;
            }
            if first_box {
                *output_box = temp_box;
            } else {
                *output_box = surrounding_box(output_box, temp_box);
                first_box = false;
            }
        }

        return false;
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }
}
