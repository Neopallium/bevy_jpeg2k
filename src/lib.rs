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
//!   App::new()
//!     .add_plugins(DefaultPlugins)
//!
//!     // Load the Jpeg 2000 asset loader plugin.
//!     .add_plugins(Jpeg2KPlugin)
//!
//!     .add_systems(Startup, setup)
//!     .run();
//! }
//!
//! fn setup(
//!   asset_server: Res<AssetServer>,
//! ) {
//!   // Load j2k, jp2, j2c, images.
//!   let image_handle: Handle<Image> = asset_server.load("example.j2k");
//!   // <Use the image handle>
//! }
//!
//! ```

use bevy::{
  app::{App, Plugin},
  asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
  prelude::*,
  utils::BoxedFuture,
};
use wgpu::{Extent3d, TextureDimension, TextureFormat};

use jpeg2k::{error, Image, ImageData};

/// Jpeg 2000 asset loader for Bevy.
#[derive(Default)]
pub struct Jpeg2KAssetLoader;

impl AssetLoader for Jpeg2KAssetLoader {
  type Asset = bevy::render::texture::Image;
  type Settings = ();
  type Error = anyhow::Error;

  fn load<'a>(
    &'a self,
    reader: &'a mut Reader,
    _settings: &'a Self::Settings,
    _load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
    Box::pin(async move {
      let mut bytes = Vec::new();
      reader.read_to_end(&mut bytes).await?;
      let image = Image::from_bytes(&bytes)?;
      let txt = image_to_texture(image)?;
      Ok(txt)
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
