use bevy::prelude::*;

mod arc;
pub use arc::ArcSampler;
mod circle;
pub use circle::CircleSampler;
mod images;
pub use images::ImageSampler;

pub trait PointSampler: Send + Sync + Copy {
  type Iter: Iterator<Item = Vec2>;
  fn samples(&self, num: usize) -> Self::Iter;

  fn upwards(&self, transform: Transform, num: usize, image: &ImageSampler, weights: Vec3) -> Vec2 {
    if weights == Vec3::ZERO {
      return Vec2::ZERO;
    }

    self
      .samples(num)
      .filter_map(|point| {
        let point = (transform * point.extend(0.0)).truncate();
        let pixel = image.sample(point)?.xyz();
        if pixel == Vec3::ZERO {
          return None;
        }

        let mag = weights.length() * pixel.length();
        let strength = weights.dot(pixel) / mag;
        Some(point.normalize_or_zero() * strength)
      })
      .sum()
  }
}
