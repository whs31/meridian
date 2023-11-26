use thiserror::Error;
use crate::geotiff::TiffParserError;
use crate::tile_storage::TileSignature;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum Error
{
  #[error("Not implemented")]
  NotImplemented,

  #[error("No such tile: {0}")]
  NoSuchTile(TileSignature),

  #[error("Network status code fail: {0} for signature {1}")]
  NetworkStatusCodeError(u16, TileSignature),

  #[error("No such object in remote: {0}")]
  NoSuchObjectInRemote(TileSignature),

  #[error("Invalid quarter directory specifier: {0}")]
  InvalidQuarterDirectorySpecifier(String),

  #[error("Missing key: {0}")]
  ConfigMissingKey(String),

  #[error(transparent)] Request(#[from] reqwest::Error),
  #[error(transparent)] Image(#[from] image::ImageError),
  #[error(transparent)] Tiff(#[from] TiffParserError),
  #[error(transparent)] ImageSize(#[from] imagesize::ImageError),
  #[error(transparent)] Positioning(#[from] meridian_positioning::positioning::errors::PositioningError),
  #[error(transparent)] Io(#[from] std::io::Error)
}