use std::env;

use bevy::{prelude::*, ui::widget::NodeImageMode};

use bevy_jpeg2k::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(Jpeg2KPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let name = env::args()
    .nth(1)
    .unwrap_or_else(|| "example.j2k".to_string());

  let image_handle = asset_server.load(name);
  // Camera
  commands.spawn(Camera2d::default());
  // root node
  commands
    .spawn(Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      justify_content: JustifyContent::SpaceBetween,
      ..Default::default()
    })
    .with_children(|parent| {
      // bevy logo (image)
      parent.spawn(ImageNode {
        image: image_handle,
        image_mode: NodeImageMode::Auto,
        ..Default::default()
      });
    });
}
