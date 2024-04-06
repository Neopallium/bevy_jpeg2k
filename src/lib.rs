//! # Bevy JPEG 2000 Asset loader.
//!
//! JPEG 2000 image loader for [Bevy](https://bevyengine.org/).
//!
//! ## Example
//!
//! ```rust
//! use bevy::prelude::*;
//!
//! use bevy_jpeg2k::*;
//!
//! fn main() {
//!   App::build()
//!     .add_plugins(DefaultPlugins)
//!
//!     // Load the Jpeg 2000 asset loader plugin.
//!     .add_plugin(Jpeg2KPlugin)
//!
//!     .add_startup_system(setup)
//!     .run();
//! }
//!
//! fn setup(
//!   asset_server: Res<AssetServer>,
//! ) {
//!   // Load j2k, jp2, j2c, images.
//!   let image_handle = asset_server.load("example.j2k");
//!   // <Use the image handle>
//! }
//!
//! ```

use bevy::{
  app::{App, Plugin},
  asset::{AddAsset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
};
use wgpu::{Extent3d, TextureDimension, TextureFormat};

use jpeg2k::{error, Image, ImageData};

/// Jpeg 2000 asset loader for Bevy.
#[derive(Default)]
pub struct Jpeg2KAssetLoader;

impl AssetLoader for Jpeg2KAssetLoader {
  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
    Box::pin(async move {
      let image = Image::from_bytes(bytes)?;
      let txt = image_to_texture(image)?;
      load_context.set_default_asset(LoadedAsset::new(txt));
      Ok(())
    })
  }

  fn extensions(&self) -> &[&str] {
    &["j2k", "jp2", "j2c", "jpc"]
  }
}

/// Try to convert a loaded Jpeg 2000 image into a Bevy `Image`.
pub fn image_to_texture(img: Image) -> error::Result<bevy::render::texture::Image> {
  let format;

  let ImageData {
    width,
    height,
    data,
    ..
  } = match img.num_components() {
    1 => {
      format = TextureFormat::R8Unorm;
      img.get_pixels(None)?
    }
    3 => {
      format = TextureFormat::Rgba8UnormSrgb;
      img.get_pixels(Some(u8::MAX))?
    }
    4 => {
      format = TextureFormat::Rgba8UnormSrgb;
      img.get_pixels(None)?
    }
    num => {
      return Err(error::Error::UnsupportedComponentsError(num));
    }
  };

  Ok(bevy::render::texture::Image::new(
    Extent3d {
      width,
      height,
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    data,
    format,
  ))
}

/// Jpeg 2000 asset plugin for Bevy.
#[derive(Default, Clone, Debug)]
pub struct Jpeg2KPlugin;

impl Plugin for Jpeg2KPlugin {
  fn build(&self, app: &mut App) {
    app.init_asset_loader::<Jpeg2KAssetLoader>();
  }
}
