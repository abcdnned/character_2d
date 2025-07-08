use bevy::prelude::*;
use nalgebra::{Point2, Vector2};
use std::f32::consts::PI;

// Fixed utility functions using nalgebra
pub fn quadratic_bezier(p0: Point2<f32>, p1: Point2<f32>, p2: Point2<f32>, t: f32) -> Point2<f32> {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    // Convert to vectors for arithmetic, then back to point
    let v0 = p0.coords;
    let v1 = p1.coords;
    let v2 = p2.coords;
    Point2::from(v0 * uu + v1 * (2.0 * u * t) + v2 * tt)
}

pub fn cubic_bezier(
    p0: Point2<f32>,
    p1: Point2<f32>,
    p2: Point2<f32>,
    p3: Point2<f32>,
    t: f32,
) -> Point2<f32> {
    // Standard cubic bezier formula: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;
    
    // Convert to vectors for arithmetic, then back to point
    let v0 = p0.coords;
    let v1 = p1.coords;
    let v2 = p2.coords;
    let v3 = p3.coords;
    
    let result = v0 * uuu + v1 * (3.0 * uu * t) + v2 * (3.0 * u * tt) + v3 * ttt;
    Point2::from(result)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}