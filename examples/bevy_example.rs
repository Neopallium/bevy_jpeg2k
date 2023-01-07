use std::env;

use bevy::prelude::*;

use bevy_jpeg2k::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(Jpeg2KPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let name = env::args()
    .nth(1)
    .unwrap_or_else(|| "example.j2k".to_string());

  let image_handle = asset_server.load(name.as_str());
  // Camera
  commands.spawn(Camera2dBundle::default());
  // root node
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        justify_content: JustifyContent::SpaceBetween,
        ..Default::default()
      },
      ..Default::default()
    })
    .with_children(|parent| {
      // bevy logo (image)
      parent.spawn(ImageBundle {
        style: Style {
          size: Size::new(Val::Auto, Val::Percent(100.0)),
          ..Default::default()
        },
        image: UiImage(image_handle),
        ..Default::default()
      });
    });
}
