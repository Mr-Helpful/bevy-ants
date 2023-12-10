#import bevy_pbr::forward_io::VertexOutput
#import bevy_render::view::View

@group(0) @binding(0) var<uniform> view: View;
@group(1) @binding(0) var<uniform> bg_color: vec4<f32>;
@group(1) @binding(1) var<uniform> stddev: f32;
@group(1) @binding(2) var bg_texture: texture_2d<f32>;
@group(1) @binding(3) var bg_sampler: sampler;

const diameter: i32 = 9;

/// Calculates the unnormalised gaussian function
fn gauss_func(x: f32, stddev: f32) -> f32 {
  return exp(-0.5 * x * x / (stddev * stddev));
}

/// Calculates a unnormalised gaussian kernel
fn blur_kernel(stddev: f32) -> array<f32, diameter> {
  let radius = (diameter - 1) / 2;
  var kernel = array<f32, diameter>();

  // slightly more efficient, kernel is symmetric
  // so we can generate both sides of the kernel at once
  // and save on calls to `gauss_func`
  for (var i = 1; i <= radius; i++) {
    let gauss = gauss_func(f32(i), stddev);
    kernel[radius - i] = gauss;
    kernel[radius + i] = gauss;
  }

  // gauss_func(0.0, stddev)
  // = exp(-0.5 * 0.0 * 0.0 / (stddev * stddev))
  // = exp(0.0)
  // = 1.0
  kernel[radius] = 1.0;

  return kernel;
}

/// Returns a blurred form of the texture
fn blurred_texture(
  pos: vec2<f32>,
  stddev: f32,
  bg_color: vec4<f32>
) -> vec4<f32> {
  let radius = (diameter - 1) / 2;
  let resolution = view.viewport.zw;
  var kernel = blur_kernel(stddev);
  var color = vec3(0.0);
  var total = 0.0;

  for (var j = -radius; j <= radius; j++) {
    for (var i = -radius; i <= radius; i++) {
      let offset = pos + vec2(f32(i), f32(j)) / resolution;
      let pixel = textureSample(bg_texture, bg_sampler, offset);

      if (i * i + j * j <= radius * radius) {
        let kernel_2d = kernel[i + radius] * kernel[j + radius];
        color += kernel_2d * pixel.rgb;
        total += kernel_2d;
      }
    }
  }

  return vec4(color / total, 1.0);
}

@fragment
fn fragment(
  mesh: VertexOutput,
) -> @location(0) vec4<f32> {
  var pixel = blurred_texture(mesh.uv, stddev, bg_color);
  if length((pixel - bg_color).xyz) < 1.0/100.0 {
    return bg_color;
  } else {
    return pixel;
  }
}
