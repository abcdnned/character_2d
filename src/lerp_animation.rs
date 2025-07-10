use bevy::prelude::*;
use nalgebra::{Point2, Vector2};
use std::f32::consts::PI;
use crate::iterpolation::*;
use crate::constants::*;

pub fn calculate_vertical_swing_cubic(t: f32, r: f32, origin: Vec3) -> (Vec2, f32) {
    // Vertical swing using cubic bezier curve
    let progress = smooth_step(t);
    
    let ox = origin.x;
    let oy = origin.y;
    // Define cubic bezier control points for a pronounced U-shaped arc
    let p0 = Point2::new(ox, oy);      // Start relative to start_pos
    let p1 = Point2::new(ox, oy + r);  // First control point (far left, slightly down)
    let p2 = Point2::new(ox - r, oy + r);   // Second control point (far right, slightly down)
    let p3 = Point2::new(ox -r, oy);    // End relative to start_pos
    
    // Calculate position using cubic bezier
    let pos = cubic_bezier(p0, p1, p2, p3, progress);
    let position = Vec2::new(pos.x, pos.y);
    
    // Rotation follows the swing direction
    let rotation = lerp(0.0, 0.0, progress);
    (position, rotation)
}