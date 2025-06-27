// metal.rs
//
// Author: David Beazley (https://dabeaz.com)
//
// Based on "Ray Tracing in One Weekend"
// (https://raytracing.github.io/books/RayTracingInOneWeekend.html

use crate::{color::*, hittable::*, material::*, ray::*, vec3::*};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = reflect(r_in.direction, rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
