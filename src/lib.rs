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

use std::path::Path;

use bevy::{
  app::{App, Plugin},
  asset::{io::Reader, AssetLoader, LoadContext, RenderAssetUsages},
  prelude::*,
};
use bevy_reflect::TypePath;
use wgpu_types::{Extent3d, TextureDimension, TextureFormat};

use jpeg2k::{error, DecodeParameters, Image, ImageData, ImagePixelData};

#[cfg(feature = "path_lod")]
use std::path::Component;

#[cfg(feature = "path_lod")]
use std::sync::Arc;

#[cfg(feature = "path_lod")]
use bevy::{
  asset::{
    io::{
      AssetReaderError, AssetSource, AssetSourceBuilder, AssetSourceId, ErasedAssetReader,
      PathStream,
    },
    AssetApp,
  },
  tasks::BoxedFuture,
};

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
fn strip_lod(path: &Path) -> (&Path, Option<u32>) {
  if let Some(top) = get_top_folder(path) {
    if top.starts_with("LOD_") {
      let lod = top[4..].parse().ok();
      // Strip the `LOD_<n>/` prefix so we read the real asset path.
      return (path.strip_prefix(top).unwrap(), lod);
    }
  }
  (path, None)
}

/// Returns the decode parameters for a path, honoring any `LOD_<n>/` prefix.
#[cfg(feature = "path_lod")]
fn lod_params(path: &Path) -> DecodeParameters {
  let (_, lod) = strip_lod(path);
  lod
    .map(|lod| DecodeParameters::new().reduce(lod))
    .unwrap_or_default()
}

/// Returns the default decode parameters (full resolution).
#[cfg(not(feature = "path_lod"))]
fn lod_params(_path: &Path) -> DecodeParameters {
  DecodeParameters::new()
}

/// A wrapping [`ErasedAssetReader`] that strips a leading `LOD_<n>/` component
/// from asset paths before delegating to the inner reader.  This lets a single
/// source file be loaded through multiple `LOD_<n>/...` asset paths.
#[cfg(feature = "path_lod")]
struct LodFileAssetReader {
  inner: Arc<dyn ErasedAssetReader>,
}

#[cfg(feature = "path_lod")]
impl LodFileAssetReader {
  fn new(inner: Arc<dyn ErasedAssetReader>) -> Self {
    Self { inner }
  }
}

#[cfg(feature = "path_lod")]
impl ErasedAssetReader for LodFileAssetReader {
  fn read<'a>(
    &'a self,
    path: &'a Path,
  ) -> BoxedFuture<'a, Result<Box<dyn Reader + 'a>, AssetReaderError>> {
    let (stripped, _) = strip_lod(path);
    self.inner.read(stripped)
  }

  fn read_meta<'a>(
    &'a self,
    path: &'a Path,
  ) -> BoxedFuture<'a, Result<Box<dyn Reader + 'a>, AssetReaderError>> {
    let (stripped, _) = strip_lod(path);
    self.inner.read_meta(stripped)
  }

  fn read_directory<'a>(
    &'a self,
    path: &'a Path,
  ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
    let (stripped, _) = strip_lod(path);
    self.inner.read_directory(stripped)
  }

  fn is_directory<'a>(
    &'a self,
    path: &'a Path,
  ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
    let (stripped, _) = strip_lod(path);
    self.inner.is_directory(stripped)
  }

  fn read_meta_bytes<'a>(
    &'a self,
    path: &'a Path,
  ) -> BoxedFuture<'a, Result<Vec<u8>, AssetReaderError>> {
    let (stripped, _) = strip_lod(path);
    self.inner.read_meta_bytes(stripped)
  }
}

/// Builds the platform-default asset source with its reader wrapped to strip a
/// leading `LOD_<n>/` component from asset paths.
#[cfg(feature = "path_lod")]
fn lod_default_source(file_path: &str) -> AssetSourceBuilder {
  let reader_path = file_path.to_string();
  AssetSourceBuilder::platform_default(file_path, None).with_reader(move || {
    let inner = AssetSource::get_default_reader(reader_path.clone())();
    Box::new(LodFileAssetReader::new(inner.into()))
  })
}

/// Registers the LOD-in-path [`AssetReader`] as the default asset source.
///
/// This plugin must be added **before** `DefaultPlugins` (or `AssetPlugin`),
/// since asset sources are built when `AssetPlugin` is added.
#[cfg(feature = "path_lod")]
#[derive(Debug, Clone)]
pub struct Jpeg2KPathLodPlugin {
  /// The asset root path, matching `AssetPlugin::file_path`.
  pub file_path: String,
}

#[cfg(feature = "path_lod")]
impl Default for Jpeg2KPathLodPlugin {
  fn default() -> Self {
    Self {
      file_path: "assets".to_string(),
    }
  }
}

#[cfg(feature = "path_lod")]
impl Plugin for Jpeg2KPathLodPlugin {
  fn build(&self, app: &mut App) {
    app.register_asset_source(
      AssetSourceId::Default,
      lod_default_source(&self.file_path),
    );
  }
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
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).await?;
    let params = lod_params(load_context.path().path());
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
