# bevy_jpeg2k

JPEG 2000 image loader for [Bevy](https://bevyengine.org/).

# Versions

- Bevy 0.10: `bevy_jpeg2k = "0.10"`
- Bevy 0.9: `bevy_jpeg2k = "0.9"`
- Bevy 0.8: `bevy_jpeg2k = "0.8"`
- Bevy 0.7: `bevy_jpeg2k = "0.7"`
- Bevy 0.6: `bevy_jpeg2k = "0.6"`
- Bevy 0.5: `bevy_jpeg2k = "0.5"`

# Example

```rust
use bevy::prelude::*;

use bevy_jpeg2k::*;

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)

    // Load the Jpeg 2000 asset loader plugin.
    .add_plugin(Jpeg2KPlugin)

    .add_startup_system(setup)
    .run();
}

fn setup(
  asset_server: Res<AssetServer>,
) {
  // Load j2k, jp2, j2c, images.
  let image_handle = asset_server.load("example.j2k");
  // <Use the image handle>
}

```
