#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0) var<uniform> bg_color: vec4<f32>;
@group(1) @binding(1) var bg_texture: texture_2d<f32>;
@group(1) @binding(2) var bg_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
  var pixel = textureSample(bg_texture, bg_sampler, mesh.uv);
  var true_color = pixel - bg_color;
  var true_signs = sign(true_color);
  
  true_color = (abs(true_color) - vec4(1.0/200.0)) * true_signs;
  if length(true_color) < 0.1 {
    true_color = vec4(0.0);
  }
  return true_color + bg_color;
}
