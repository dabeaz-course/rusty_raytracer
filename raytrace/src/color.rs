// color.rs
//
// Author: David Beazley (https://dabeaz.com)
//
// Based on "Ray Tracing in One Weekend"
// (https://raytracing.github.io/books/RayTracingInOneWeekend.html

use crate::{interval::*, random::*};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
    pub fn zero() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    pub fn random(min: f64, max: f64) -> Color {
        Color::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }
}
impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, other: Self) -> Self::Output {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl std::ops::Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        Color::new(self * other.r, self * other.g, self * other.b)
    }
}

const INTENSITY: Interval = Interval::new(0.0, 0.999);

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color) {
    let r = linear_to_gamma(pixel_color.r);
    let g = linear_to_gamma(pixel_color.g);
    let b = linear_to_gamma(pixel_color.b);

    let rbyte = (256.0 * INTENSITY.clamp(r)) as u8;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as u8;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as u8;

    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap();
}

fn linear_to_gamma(x: f64) -> f64 {
    if x >= 0.0 {
        x.sqrt()
    } else {
        0.0
    }
}
