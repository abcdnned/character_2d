use bevy::prelude::*;
use nalgebra::{Point2, Vector2};
use std::f32::consts::PI;
use crate::iterpolation::*;
use crate::constants::*;

pub fn calculate_vertical_swing_cubic(t: f32, r: f32) -> (Vec2, f32) {
    // Vertical swing using cubic bezier curve
    let progress = smooth_step(t);
    
    let x1 = r;
    let y1 = -r * 0.2;
    let x2 = r;
    let y2 = r * 1.3;
    let x3 = -r;
    let y3 = r * 1.3;
    let x4 = -r;
    let y4 = -r * 0.3;
    let p0 = Point2::new(x1, y1);          // Start point
    let p1 = Point2::new(x2, y2);          // First control point
    let p2 = Point2::new(x3, y3);          // Second control point
    let p3 = Point2::new(x4, y4);          // End point
    
    // Calculate position using cubic bezier
    let pos = cubic_bezier(p0, p1, p2, p3, progress);
    let position = Vec2::new(pos.x, pos.y);
    
    // Rotation follows the swing direction
    let rotation = lerp(-PI * 0.6, PI * 0.6, progress);
    (position, rotation)
}