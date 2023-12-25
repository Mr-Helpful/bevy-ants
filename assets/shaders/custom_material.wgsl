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
      var pixel = textureSample(bg_texture, bg_sampler, offset);

      if (i * i + j * j <= radius * radius) {
        let kernel_2d = kernel[i + radius] * kernel[j + radius];
        color += kernel_2d * pixel.rgb;
        total += kernel_2d;
      }
    }
  }

  return vec4(color / total, 1.0);
}

fn cubic_fade(x: f32) -> f32 {
  return x * x * (x * -2.0 + 3.0);
}

fn quintic_fade(x: f32) -> f32 {
  return x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
}

/// A function that steps from y = 1.0 -> 0.0 from x = w -> 1.0
/// 
/// step_down x w
///   { assert 0 <= w < 1 }
///   |      x < w = 1
///   | w <= x < 1 = lerp w 0 ((x - w)/w)
///   | 1 <= x     = 0
/// 
/// i.e. a shape very much like:
/// __________  1
///    |     |\ |
///    |     | \|
///    |     |  ___________
///    0     width
/// see [desmos](https://www.desmos.com/calculator/tj64xjpip6)
fn step_down(x: f32, width: f32) -> f32 {
  return clamp((1.0 - x) / (1.0 - width), 0.0, 1.0);
}

/// A box with smoothed off corners, starting at w
/// This looks a little bit like keycaps on external keyboards
/// Look at `assets/documentation/shaders/edge_falloff_shader.webm`
fn box_fade(uv: vec2<f32>, width: f32) -> f32 {
  // convert to [-1, 1] x [-1, 1]
  let xy = 2.0 * uv - 1.0;
  // calculate the box fade
  let u = quintic_fade(step_down(abs(xy.x), width));
  let v = quintic_fade(step_down(abs(xy.y), width));
  return u * v;
}

const W: f32 = 0.99;

fn fade_on_edge(
  uv: vec2<f32>,
  pixel: vec4<f32>,
  color: vec4<f32>,
) -> vec4<f32> {
  let t = box_fade(uv, W);
  return (1.0 - t) * color + t * pixel;
}

fn biased_round(x: vec4<f32>, f: vec4<f32>) -> vec4<f32> {
  let frac_lte = step(fract(x), f);
  return floor(x) * frac_lte + ceil(x) * (1.0 - frac_lte);
}

@fragment
fn fragment(
  mesh: VertexOutput,
) -> @location(0) vec4<f32> {
  var pixel = blurred_texture(mesh.uv, stddev, bg_color);
  pixel = fade_on_edge(mesh.uv, pixel, bg_color);
  if length((pixel - bg_color).xyz) < 1.0/100.0 {
    return bg_color;
  } else {
    return biased_round(pixel * 256.0, vec4(0.75))/256.0;
  }
}
