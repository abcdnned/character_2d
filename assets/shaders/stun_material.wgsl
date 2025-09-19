#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var base_color = material_color;

    // Sample the texture if available
    if (textureNumLevels(material_color_texture) > 0u) {
        base_color = base_color * textureSample(material_color_texture, material_color_sampler, mesh.uv);
    }

    // Apply red tint for stun effect
    // Mix the original color with red to create a stunned appearance
    let red_tint = vec4<f32>(1.0, 0.2, 0.2, 1.0);
    let stunned_color = mix(base_color, red_tint, 0.7);

    return stunned_color;
}