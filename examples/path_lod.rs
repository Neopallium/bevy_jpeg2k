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
  commands.spawn(Camera2d::default());

  // Load the same source file at several resolutions using the `LOD_<n>/`
  // path prefix.  Each distinct path produces its own `Handle<Image>` decoded
  // at a different reduction level (each increment of `n` halves the decoded
  // dimensions), so the images are shown at full, half, and quarter size.
  let images = [
    asset_server.load("example.j2k"),
    asset_server.load("LOD_1/example.j2k"),
    asset_server.load("LOD_2/example.j2k"),
  ];

  commands
    .spawn(Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      justify_content: JustifyContent::SpaceAround,
      align_items: AlignItems::Center,
      ..Default::default()
    })
    .with_children(|parent| {
      for image in images {
        parent.spawn(ImageNode {
          image,
          image_mode: NodeImageMode::Auto,
          ..Default::default()
        });
      }
    });
}
