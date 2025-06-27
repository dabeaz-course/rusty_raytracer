// hittable_list.rs
//
// Author: David Beazley (https://dabeaz.com)
//
// Based on "Ray Tracing in One Weekend"
// (https://raytracing.github.io/books/RayTracingInOneWeekend.html

use crate::{hittable::*, interval::*, ray::*};

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: impl Hittable + 'a) {
        self.objects.push(Box::new(object));
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max;
        for object in self.objects.iter() {
            if let Some(hitrec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hitrec.t;
                rec = Some(hitrec);
            }
        }
        rec
    }
}
