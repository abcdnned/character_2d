use bevy::prelude::*;
use nalgebra::{Point2, Vector2};
use std::f32::consts::PI;
use crate::iterpolation::*;
use crate::constants::*;

pub fn calculate_vertical_swing_cubic(t: f32, r: f32) -> (Vec2, f32) {
    // Vertical swing using cubic bezier curve
    let progress = smooth_step(t);
    
    // Define cubic bezier control points for a pronounced U-shaped arc
    let p0 = Point2::new(0.0, 0.0);      // Start relative to start_pos
    let p1 = Point2::new(00.0, r);  // First control point (far left, slightly down)
    let p2 = Point2::new(-r, r);   // Second control point (far right, slightly down)
    let p3 = Point2::new(-r, 000.0);    // End relative to start_pos
    
    // Calculate position using cubic bezier
    let pos = cubic_bezier(p0, p1, p2, p3, progress);
    let position = Vec2::new(pos.x, pos.y);
    
    // Rotation follows the swing direction
    let rotation = lerp(0.0, 0.0, progress);
    (position, rotation)
}