use bevy::{prelude::*, render::render_resource::TextureFormat};

pub(crate) fn is_vec4_u8_format(format: &TextureFormat) -> bool {
  use TextureFormat::*;
  matches!(
    format,
    Bgra8Unorm | Bgra8UnormSrgb | Rgba8Unorm | Rgba8UnormSrgb
  )
}

pub struct ImageSampler<'a>(&'a Image);

impl<'a> TryFrom<&'a Image> for ImageSampler<'a> {
  type Error = String;
  fn try_from(value: &'a Image) -> Result<Self, Self::Error> {
    let format = &value.texture_descriptor.format;
    if !is_vec4_u8_format(format) {
      return Err(format!("Unsupported format {format:?}"));
    }
    Ok(Self(value))
  }
}

impl<'a> ImageSampler<'a> {
  fn sample_unchecked(&self, i: usize) -> Vec4 {
    let mut pixels = [0.0; 4];
    for (j, v) in pixels.iter_mut().enumerate() {
      *v = (self.0.data[i + j] as f32) / 256.0;
    }
    Vec4::from_array(pixels)
  }

  fn sample_square(&self, [lx, ty]: [usize; 2], [u, v]: [f32; 2]) -> Vec4 {
    let w = self.0.width() as usize;
    let (rx, by) = (lx + 1, ty + 1);
    let (iu, iv) = (1.0 - u, 1.0 - v);
    [
      (lx, ty, iu * iv),
      (rx, ty, u * iv),
      (lx, by, iu * v),
      (rx, by, u * v),
    ]
    .into_iter()
    .map(|(x, y, f)| {
      let i = (y * w + x) * 4;
      f * self.sample_unchecked(i)
    })
    .sum()
  }

  pub fn sample(&self, Vec2 { x, y }: Vec2) -> Option<Vec4> {
    let (w, h) = (self.0.width() as isize, self.0.height() as isize);
    let cx = x.floor() as isize;
    let cy = y.floor() as isize;
    if (cx + 1 >= w) || (cy + 1 >= h) {
      return None;
    }

    Some(self.sample_square(
      [cx.try_into().ok()?, cy.try_into().ok()?],
      [x.fract(), y.fract()],
    ))
  }
}
