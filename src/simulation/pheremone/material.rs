use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderRef},
  sprite::Material2d,
};

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct BlurMaterial {
  #[uniform(0)]
  pub(crate) background: Color,
  #[uniform(1)]
  pub(crate) stddev: f32,
  #[texture(2)]
  #[sampler(3)]
  pub(crate) texture: Handle<Image>,
}

impl Material2d for BlurMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/custom_material.wgsl".into()
  }
}
