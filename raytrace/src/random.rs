// random.rs
//
// Author: David Beazley (https://dabeaz.com)
//
// Based on "Ray Tracing in One Weekend"
// (https://raytracing.github.io/books/RayTracingInOneWeekend.html

pub fn random_f64() -> f64 {
    fastrand::f64()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
