// sphere.rs
//
// Author: David Beazley (https://dabeaz.com)
//
// Based on "Ray Tracing in One Weekend"
// (https://raytracing.github.io/books/RayTracingInOneWeekend.html

use crate::{hittable::*, interval::*, material::*, ray::*, vec3::*};

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub mat: Box<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, mat: impl Material + 'a) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            mat: Box::new(mat),
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(p, outward_normal, root, r, &*self.mat))
    }
}
