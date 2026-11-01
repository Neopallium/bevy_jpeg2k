# bevy_jpeg2k

JPEG 2000 image loader for [Bevy](https://bevyengine.org/).

# Versions

- Bevy 0.20: `bevy_jpeg2k = "0.20.1"`
- Bevy 0.19: `bevy_jpeg2k = "0.19"`
- Bevy 0.18: `bevy_jpeg2k = "0.18"`
- Bevy 0.17: `bevy_jpeg2k = "0.17"`
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

# Level of Detail (LOD)

JPEG 2000 supports decoding a single source image at reduced resolutions, which
can be used to save memory and load time when a lower-resolution version of an
image is sufficient.

LOD support is opt-in and requires the `path_lod` feature:

```toml
bevy_jpeg2k = { version = "0.20.1", features = [ "path_lod" ] }
```

To request a reduced-resolution decode, prefix the asset path with `LOD_<n>/`,
where `n` is the reduction level.  Each increment of `n` halves the decoded
image dimensions (a reduction of `n` decodes at `1 / 2^n` resolution).  The
source file itself lives at the path *without* the `LOD_<n>/` prefix.

```rust
// Load `textures/example.j2k` at full resolution.
let full = asset_server.load("textures/example.j2k");

// Load `textures/example.j2k` at quarter (1/2^2) resolution.
let quarter = asset_server.load("LOD_2/textures/example.j2k");
```

Each distinct `LOD_<n>/...` path produces its own `Handle<Image>`, so a single
source file can be loaded at several resolutions simultaneously.

To enable LOD-in-path, the `Jpeg2KPathLodPlugin` must be added **before**
`DefaultPlugins` (it registers a custom asset reader, and asset sources are
built when `AssetPlugin` is added as part of `DefaultPlugins`):

```rust
App::new()
  .add_plugins(Jpeg2KPathLodPlugin::default())
  .add_plugins(DefaultPlugins)
  .add_plugins(Jpeg2KPlugin)
  // ...
  .run();
```

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
