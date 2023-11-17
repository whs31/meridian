#[derive(Debug)]
pub enum Error
{
  NoSuchTile,
  NetworkFailure,
  InvalidQuarterDirectorySpecifier,
  ConfigMissingKey
}