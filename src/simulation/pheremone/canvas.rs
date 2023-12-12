use bevy::{
  prelude::*,
  render::{
    camera::RenderTarget,
    render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    view::RenderLayers,
  },
  sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

pub use super::material::BlurMaterial;

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct CanvasMarker;

#[derive(Bundle, Default)]
pub struct Canvas {
  marker: CanvasMarker,
  canvas: MaterialMesh2dBundle<BlurMaterial>,
  image: Handle<Image>,
  layers: RenderLayers,
}

impl Canvas {
  fn new(
    mut meshes: ResMut<Assets<Mesh>>,
    image: Handle<Image>,
    texture: Handle<BlurMaterial>,
    size: Extent3d,
    layer: u8,
  ) -> Self {
    let size = Vec2::new(size.width as f32, size.height as f32);
    Self {
      marker: CanvasMarker,
      canvas: MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(size).into()).into(),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        material: texture,
        ..default()
      },
      image,
      layers: RenderLayers::from_layers(&[0, layer]),
    }
  }
}

#[derive(Component, Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct CanvasCameraMarker;

#[derive(Bundle, Default)]
pub struct CanvasCamera {
  marker: CanvasCameraMarker,
  camera: Camera2dBundle,
  layers: RenderLayers,
}

impl CanvasCamera {
  fn new(image: Handle<Image>, layer: u8) -> Self {
    Self {
      marker: CanvasCameraMarker,
      camera: Camera2dBundle {
        camera: Camera {
          target: RenderTarget::Image(image),
          ..default()
        },
        ..default()
      },
      layers: RenderLayers::layer(layer),
    }
  }
}

/// Initialises the pheremone canvas on render layer `layer`
/// with standard deviation `stddev` (which controls how fast pheremones fade)
fn setup_canvas(
  In((layer, stddev)): In<(u8, f32)>,
  mut commands: Commands,
  mut materials: ResMut<Assets<BlurMaterial>>,
  mut images: ResMut<Assets<Image>>,
  meshes: ResMut<Assets<Mesh>>,
  bg_color: Res<ClearColor>,
  windows: Query<&Window>,
) {
  let window = windows.single();

  // Get an image handle to used to link the camera to the background
  let size = Extent3d {
    width: window.physical_width(),
    height: window.physical_height(),
    ..default()
  };
  let mut image = Image::new_fill(
    size,
    TextureDimension::D2,
    &bg_color.0.as_rgba_u8(),
    TextureFormat::Bgra8UnormSrgb,
  );
  image.texture_descriptor.usage =
    TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
  let image = images.add(image);

  // Spawn a camera to render the recursive background
  commands.spawn(CanvasCamera::new(image.clone(), layer));

  // Spawn the background to recursively render
  let texture = materials.add(BlurMaterial {
    stddev,
    // render custom material from our common image
    texture: image.clone(),
    background: bg_color.0,
  });
  commands.spawn(Canvas::new(meshes, image, texture, size, layer));
}

#[derive(Clone, Copy, Debug)]
pub struct PheremonePlugin {
  pub layer: u8,
  pub stddev: f32,
}

impl PheremonePlugin {
  pub fn new(layer: u8, stddev: f32) -> Self {
    Self { layer, stddev }
  }
}

impl Plugin for PheremonePlugin {
  fn build(&self, app: &mut App) {
    let params = (self.layer, self.stddev);
    app
      .add_plugins(Material2dPlugin::<BlurMaterial>::default())
      .add_systems(Startup, (move || params).pipe(setup_canvas));
  }
}
