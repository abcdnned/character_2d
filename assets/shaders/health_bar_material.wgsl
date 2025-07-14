// shaders/health_bar_material.wgsl

// Draws a simple health bar
#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> fill_ratio: vec4<f32>;
@group(1) @binding(1) var<uniform> health_color: vec4<f32>;
@group(1) @binding(2) var<uniform> border_color: vec4<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // half size of the UI node
    let half_size = 0.5 * in.size;

    // position relative to the center of the UI node
    let p = in.uv * in.size - half_size;

    // thickness of the border closest to the current position
    let b = vec2(
        select(in.border_widths.x, in.border_widths.z, 0. < p.x),
        select(in.border_widths.y, in.border_widths.w, 0. < p.y)
    );

    // select radius for the nearest corner
    let rs = select(in.border_radius.xy, in.border_radius.wz, 0.0 < p.y);
    let radius = select(rs.x, rs.y, 0.0 < p.x);

    // distance along each axis from the corner
    let d = half_size - abs(p);

    // if the distance to the edge from the current position on any axis 
    // is less than the border width on that axis then the position is within 
    // the border and we return the border color
    if d.x < b.x || d.y < b.y {
        // select radius for the nearest corner
        let rs = select(in.border_radius.xy, in.border_radius.wz, 0.0 < p.y);
        let radius = select(rs.x, rs.y, 0.0 < p.x);

        // determine if the point is inside the curved corner and return the corresponding color
        let q = radius - d;
        if radius < min(max(q.x, q.y), 0.0) + length(vec2(max(q.x, 0.0), max(q.y, 0.0))) {
            return vec4(0.0);
        } else {
            return border_color;
        }
    }

    // Fill the health bar based on fill_ratio
    if in.uv.x < fill_ratio.x {
        return health_color;
    } else {
        // Empty part of the health bar - slightly transparent dark color
        return vec4(0.2, 0.2, 0.2, 0.5);
    }
}