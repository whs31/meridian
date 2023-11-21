#[derive(Debug)]
#[allow(dead_code)]
pub enum Error
{
  NotImplemented,
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