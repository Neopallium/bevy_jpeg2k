# bevy_jpeg2k

JPEG 2000 image loader for [Bevy](https://bevyengine.org/).

# Versions

- Bevy 0.16: `bevy_jpeg2k = "0.16"`
- Bevy 0.15: `bevy_jpeg2k = "0.15"`
- Bevy 0.14: `bevy_jpeg2k = "0.14"`
- Bevy 0.13: `bevy_jpeg2k = "0.13"`
- Bevy 0.12: `bevy_jpeg2k = "0.12"`
- Bevy 0.11: `bevy_jpeg2k = "0.11"`
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
  App::new()
    .add_plugins(DefaultPlugins)

    // Load the Jpeg 2000 asset loader plugin.
    .add_plugins(Jpeg2KPlugin)

    .add_systems(Startup, setup)
    .run();
}

fn setup(
  asset_server: Res<AssetServer>,
) {
  // Load j2k, jp2, j2c, images.
  let image_handle: Handle<Image> = asset_server.load("example.j2k");
  // <Use the image handle>
}

```
