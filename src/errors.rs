use thiserror::Error;
use crate::geotiff::TiffParserError;
use crate::positioning::geocoordinate::GeoCoordinate;
use crate::tile_storage::TileSignature;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum Error
{
  #[error("Not implemented")]
  NotImplemented,

  #[error("No such tile: {0}")]
  NoSuchTile(TileSignature),

  #[error("Network failure: {0}")]
  NetworkFailure(reqwest::Error),

  #[error("Network status code fail: {0} for signature {1}")]
  NetworkStatusCodeError(u16, TileSignature),

  #[error("No such object in remote: {0}")]
  NoSuchObjectInRemote(TileSignature),

  #[error("Invalid quarter directory specifier: {0}")]
  InvalidQuarterDirectorySpecifier(String),

  #[error("Missing key: {0}")]
  ConfigMissingKey(String),

  #[error("Failed to read file: {0}")]
  TiffError(TiffParserError),

  #[error("Failed to read image size: {0}")]
  ImageSizeError(imagesize::ImageError),

  #[error("Operation on invalid coordinate: {0}")]
  OperationOnInvalidCoordinate(GeoCoordinate),

  #[error("Operation on invalid coordinate pair: {0}, {1}")]
  OperationOnInvalidCoordinatePair(GeoCoordinate, GeoCoordinate),

  #[error("Failed to save image: {0}")]
  ImageSaveFailure(image::ImageError),

  #[error("Failed to create file: {0}")]
  FileCreationFailure(std::io::Error),

  #[error("Failed to write to file: {0}")]
  WriteToFileFailure(String)
}