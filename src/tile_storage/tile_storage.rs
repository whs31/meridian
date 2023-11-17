//static

use crate::errors::Error;
use crate::tile_storage::tile_identity::TileIdentity;
use crate::tile_storage::tile_signature::TileSignature;

pub struct TileStorage
{
  table: std::collections::HashMap<TileSignature, Box<TileIdentity>>
}

impl TileStorage
{
  pub fn new() -> Self
  {
    Self {
      table: std::collections::HashMap::new()
    }
  }

  pub fn is_available(&self, signature: TileSignature) -> bool
  {
    return self.table.contains_key(&signature);
  }

  pub fn get(&self, signature: TileSignature) -> Result<&TileIdentity, Error>
  {
    return match self.table.get(&signature) {
      None => { Err(Error::NoSuchTile) },
      Some(x) => { Ok(x.as_ref()) }
    }
  }
}