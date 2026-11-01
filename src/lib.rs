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

#[cfg(feature = "path_lod")]
use std::path::{Component, Path, PathBuf};

use bevy::{
  app::{App, Plugin},
  asset::{io::Reader, AssetLoader, LoadContext, RenderAssetUsages},
  prelude::*,
};
use bevy_reflect::TypePath;
use wgpu_types::{Extent3d, TextureDimension, TextureFormat};

use jpeg2k::{error, DecodeParameters, Image, ImageData, ImagePixelData};

#[cfg(feature = "path_lod")]
fn get_top_folder(path: &Path) -> Option<&str> {
  path.components().next().and_then(|top| {
    if let Component::Normal(top) = top {
      top.to_str()
    } else {
      None
    }
  })
}

/// Strip a leading `LOD_<n>/` component from an asset path, returning the real
/// asset path along with the requested reduction level.
///
/// Level of detail is encoded in the asset path: loading `LOD_2/textures/foo.j2k`
/// reads the source file at `textures/foo.j2k` and decodes it at 1/2^2 (quarter)
/// resolution.  Each increment of `n` halves the decoded image dimensions.
/// Paths without a `LOD_<n>/` prefix are returned unchanged with `None`.
#[cfg(feature = "path_lod")]
fn strip_lod(path: &Path) -> (PathBuf, Option<u32>) {
  if let Some(top) = get_top_folder(path) {
    if top.starts_with("LOD_") {
      let lod = top[4..].parse().ok();
      // Strip the `LOD_<n>/` prefix so we read the real asset path.
      return (path.strip_prefix(top).unwrap().to_path_buf(), lod);
    }
  }
  (path.to_path_buf(), None)
}

/// Read the source image bytes and the decode parameters to use.
///
/// When the `path_lod` feature is enabled this also honors the `LOD_<n>/`
/// path prefix, reading the real (stripped) asset path and decoding it at
/// reduced resolution.  Without the feature the reader is used directly.
#[cfg(feature = "path_lod")]
async fn read_image_bytes(
  reader: &mut dyn Reader,
  load_context: &mut LoadContext<'_>,
) -> Result<(Vec<u8>, DecodeParameters), anyhow::Error> {
  let (path, lod) = strip_lod(load_context.path().path());
  match lod {
    Some(lod) => {
      // The asset was requested through a `LOD_<n>/...` path; read the real
      // (stripped) source path rather than the (possibly missing) prefixed one.
      let bytes = load_context.read_asset_bytes(path).await?;
      Ok((bytes, DecodeParameters::new().reduce(lod)))
    }
    None => {
      let mut bytes = Vec::new();
      reader.read_to_end(&mut bytes).await?;
      Ok((bytes, DecodeParameters::new()))
    }
  }
}

/// Read the source image bytes and the decode parameters to use.
#[cfg(not(feature = "path_lod"))]
async fn read_image_bytes(
  reader: &mut dyn Reader,
  _load_context: &mut LoadContext<'_>,
) -> Result<(Vec<u8>, DecodeParameters), anyhow::Error> {
  let mut bytes = Vec::new();
  reader.read_to_end(&mut bytes).await?;
  Ok((bytes, DecodeParameters::new()))
}

/// Jpeg 2000 asset loader for Bevy.
#[derive(Default, TypePath)]
pub struct Jpeg2KAssetLoader;

impl AssetLoader for Jpeg2KAssetLoader {
  type Asset = bevy::image::Image;
  type Settings = ();
  type Error = anyhow::Error;

  async fn load(
    &self,
    reader: &mut dyn Reader,
    _settings: &Self::Settings,
    load_context: &mut LoadContext<'_>,
  ) -> Result<Self::Asset, Self::Error> {
    let (bytes, params) = read_image_bytes(reader, load_context).await?;
    let image = Image::from_bytes_with(&bytes, params)?;
    let txt = image_to_texture(image)?;
    Ok(txt)
  }

  fn extensions(&self) -> &[&str] {
    &["j2k", "jp2", "j2c", "jpc"]
  }
}

/// Try to convert a loaded Jpeg 2000 image into a Bevy `Image`.
pub fn image_to_texture(img: Image) -> error::Result<bevy::image::Image> {
  let ImageData {
    width,
    height,
    data,
    ..
  } = match img.num_components() {
    1 | 4 => img.get_pixels(None)?,
    3 => img.get_pixels(Some(u32::MAX))?,
    num => {
      return Err(error::Error::UnsupportedComponentsError(num));
    }
  };

  let (format, data) = match data {
    ImagePixelData::L8(data) => (TextureFormat::R8Unorm, data),
    ImagePixelData::La8(data) => (TextureFormat::Rg8Unorm, data),
    ImagePixelData::Rgba8(data) => (TextureFormat::Rgba8UnormSrgb, data),
    _ => {
      return Err(error::Error::UnknownFormatError(format!(
        "Unsupported pixel data."
      )));
    }
  };

  Ok(bevy::image::Image::new(
    Extent3d {
      width,
      height,
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    data,
    format,
    RenderAssetUsages::all(),
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
