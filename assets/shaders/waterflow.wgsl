#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::view,
    pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
    pbr_functions as fns,
    pbr_bindings,
     mesh_view_bindings::globals,
}
#import bevy_core_pipeline::tonemapping::tone_mapping

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var bar_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var bar_sampler: sampler;


@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var<uniform> fill: f32; // 0..1

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let speed = 0.5;

    let flow_uv = in.uv + vec2<f32>(globals.time * speed, 0.0);

    let tex_color = textureSample(bar_texture, bar_sampler, flow_uv);

    let mask = step(in.uv.x, fill);

    return vec4<f32>(tex_color.rgb, tex_color.a * mask);

}







