// main.rs
//
// Author: David Beazley (https://dabeaz.com)
//
// Based on "Ray Tracing in One Weekend"
// (https://raytracing.github.io/books/RayTracingInOneWeekend.html

#[macro_use]
mod camera;
mod color;
mod dialectric;
mod hittable;
mod hittable_list;
mod interval;
mod lambertian;
mod material;
mod metal;
mod random;
mod ray;
mod sphere;
mod vec3;

use crate::{
    camera::*, color::*, dialectric::*, hittable_list::*, lambertian::*, metal::*, random::*,
    sphere::*, vec3::*,
};

fn main() {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if choose_mat < 0.8 {
                let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                let mat = Lambertian::new(albedo);
                world.add(Sphere::new(center, 0.2, mat));
            } else if choose_mat < 0.95 {
                let albedo = Color::random(0.5, 1.0);
                let fuzz = random_range(0.0, 0.5);
                let mat = Metal::new(albedo, fuzz);
                world.add(Sphere::new(center, 0.2, mat));
            } else {
                let mat = Dialectric::new(1.5);
                world.add(Sphere::new(center, 0.2, mat));
            };
        }
    }
    let material1 = Dialectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let cam = Camera! {
	aspect_ratio : 16.0/9.0,
	image_width : 400,
	samples_per_pixel : 500,
	max_depth : 50,
	vfov : 20.0,
	lookfrom : Point3::new(13.0, 2.0, 3.0),
	lookat : Point3::new(0.0, 0.0, 0.0),
	vup : Vec3::new(0.0, 1.0, 0.0),
	defocus_angle : 0.6,
	focus_dist : 10.0,
    };

    cam.render(std::io::stdout(), &world);
}
