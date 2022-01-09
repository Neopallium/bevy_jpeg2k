# bevy_jpeg2k

JPEG 2000 image loader for [Bevy](https://bevyengine.org/).

```rust
use bevy::prelude::*;

use jpeg2k::loader::*;

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)

    // Load the Jpeg 2000 asset loader plugin.
    .add_plugin(Jpeg2KPlugin)

    .add_startup_system(setup.system())
    .run();
}

fn setup(
  asset_server: Res<AssetServer>,
) {
  // Load j2k, jp2, j2c, images.
  let texture_handle = asset_server.load("example.j2k");
  // <Use the texture handle>
}

```