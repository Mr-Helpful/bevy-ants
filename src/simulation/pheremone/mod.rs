use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, camera::RenderTarget}};

fn setup(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
) {
  let size = Extent3d {
    width: 1500, height: 1024, ..default()
  };

  let mut image = Image {
    texture_descriptor: TextureDescriptor {
        label: None,
        size,
        dimension: TextureDimension::D2,
        format: TextureFormat::Bgra8UnormSrgb,
        mip_level_count: 1,
        sample_count: 1,
        usage: TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    },
    ..default()
};
  image.resize(size);
  let image_handle = images.add(image);

  commands.spawn(Camera2dBundle {
    camera: Camera {
      target: RenderTarget::Image(image_handle.clone()),
      order: -1,
      ..default()
    },
    ..default()
  });

  commands.spawn(Camera2dBundle::default());
  
  commands.spawn(SpriteBundle {
    texture: image_handle,
    transform: Transform::from_xyz(0.0, 0.0, -1.0),
    ..default()
  });
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct PheremonePlugin;

impl Plugin for PheremonePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
  }
}