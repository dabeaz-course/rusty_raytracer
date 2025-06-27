// hittable.rs

use crate::{interval::*, material::*, ray::*, vec3::*};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        t: f64,
        r: &Ray,
        mat: &'a dyn Material,
    ) -> HitRecord<'a> {
        let front_face = dot(r.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
