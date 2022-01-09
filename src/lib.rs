//! # Bevy Asset loader.
//!
//! Provides Jpeg 2000 support for Bevy engine.
//!
//! ## Example
//!
//! ```rust
//! use bevy::prelude::*;
//!
//! use jpeg2k::loader::*;
//!
//! fn main() {
//!   App::build()
//!     .add_plugins(DefaultPlugins)
//!
//!     // Load the Jpeg 2000 asset loader plugin.
//!     .add_plugin(Jpeg2KPlugin)
//!
//!     .add_startup_system(setup.system())
//!     .run();
//! }
//!
//! fn setup(
//!   asset_server: Res<AssetServer>,
//! ) {
//!   // Load j2k, jp2, j2c, images.
//!   let texture_handle = asset_server.load("example.j2k");
//!   // <Use the texture handle>
//! }
//!
//! ```

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    utils::BoxedFuture,
};

use jpeg2k::{Image, error};

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

/// Try to convert a loaded Jpeg 2000 image into a Bevy `Texture`.
pub fn image_to_texture(img: Image) -> error::Result<Texture> {
  use bevy::render::texture::*;
  let comps = img.components();
  let (width, height) = comps.get(0).map(|c| (c.width(), c.height()))
    .ok_or_else(|| error::Error::UnsupportedComponentsError(0))?;
  let format;

  let data = match comps {
    [r] => {
      format = TextureFormat::R8Unorm;
      r.data().iter().map(|r| *r as u8).collect()
    }
    [r, g, b] => {
      let len = (width * height) as usize;
      let mut pixels = Vec::with_capacity(len * 4);

      format = TextureFormat::Rgba8UnormSrgb;
      for (r, (g, b)) in r.data().iter().zip(g.data().iter().zip(b.data().iter())) {
        pixels.extend_from_slice(&[*r as u8, *g as u8, *b as u8, u8::MAX]);
      }
      pixels
    }
    [r, g, b, a] => {
      let len = (width * height) as usize;
      let mut pixels = Vec::with_capacity(len * 4);

      format = TextureFormat::Rgba8UnormSrgb;
      for (r, (g, (b, a))) in r.data().iter().zip(g.data().iter().zip(b.data().iter().zip(a.data().iter()))) {
        pixels.extend_from_slice(&[*r as u8, *g as u8, *b as u8, *a as u8]);
      }
      pixels
    }
    _ => {
      return Err(error::Error::UnsupportedComponentsError(img.num_components()));
    }
  };

  Ok(Texture::new(
    Extent3d::new(width, height, 1),
    TextureDimension::D2,
    data, format,
  ))
}

/// Jpeg 2000 asset plugin for Bevy.
#[derive(Default, Clone, Debug)]
pub struct Jpeg2KPlugin;

impl Plugin for Jpeg2KPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_asset_loader::<Jpeg2KAssetLoader>()
      ;
  }
}