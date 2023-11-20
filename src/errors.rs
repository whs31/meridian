#[derive(Debug)]
#[allow(dead_code)]
pub enum Error
{
  NoSuchTile,
  NetworkFailure,
  InvalidQuarterDirectorySpecifier,
  ConfigMissingKey,
  TiffError,
  InvalidCoordinate,
  ImageSizeError,
  EmptyPath,
  OperationOnInvalidCoordinate,
  ImageSaveFailure,
  FileCreationFailure,
  WriteToFileFailure
}