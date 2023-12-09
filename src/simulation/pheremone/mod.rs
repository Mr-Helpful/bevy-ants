use bevy::{
  prelude::*,
  render::{
    camera::RenderTarget,
    render_resource::{
      AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
      TextureUsages,
    },
  },
  sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct BlurMaterial {
  #[uniform(0)]
  background: Color,
  #[uniform(1)]
  stddev: f32,
  #[texture(2)]
  #[sampler(3)]
  texture: Handle<Image>,
}

impl Material2d for BlurMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/custom_material.wgsl".into()
  }
}

fn setup(
  mut commands: Commands,
  mut materials: ResMut<Assets<BlurMaterial>>,
  mut images: ResMut<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  bg_color: Res<ClearColor>,
) {
  // Get an image handle to used to link the camera to the background
  let size = Extent3d {
    width: 1500,
    height: 1024,
    ..default()
  };
  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: None,
      size,
      sample_count: 1,
      mip_level_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::Bgra8UnormSrgb,
      usage: TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    ..default()
  };
  image.resize(size);
  let image_handle = images.add(image);

  // Spawn a camera to render the recursive background
  commands.spawn(Camera2dBundle {
    camera: Camera {
      // set camera to render to our common image
      target: RenderTarget::Image(image_handle.clone()),
      ..default()
    },
    ..default()
  });

  // Spawn the background to recursively render
  let size_vec = Vec2::new(size.width as f32, size.height as f32);
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(shape::Quad::new(size_vec).into()).into(),
    material: materials.add(BlurMaterial {
      stddev: 2.0,
      // render custom material from our common image
      texture: image_handle.clone(),
      background: bg_color.0,
    }),
    ..default()
  });
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct PheremonePlugin;

impl Plugin for PheremonePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(Material2dPlugin::<BlurMaterial>::default())
      .add_systems(Startup, setup);
  }
}
