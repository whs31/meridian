use std::collections::{HashMap};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::errors::Error;
use crate::tile_storage::TileLimiter;
use crate::tile_storage::NetworkFetcher;
use crate::tile_storage::TileIdentity;
use crate::tile_storage::TileSignature;
use crate::utils::StaticHeapObject;

pub static STORAGE: StaticHeapObject<TileStorage> = Lazy::new(
  || { Mutex::new(Box::new(TileStorage::new())) }
);

pub struct TileStorage
{
  table: HashMap<TileSignature, Box<TileIdentity>>,
  network: NetworkFetcher,
  limiter: TileLimiter
}

impl TileStorage
{
  pub fn new() -> Self
  {
    Self {
      table: HashMap::new(),
      network: NetworkFetcher::new(),
      limiter: TileLimiter::new(25)
    }
  }

  pub fn get(&mut self, signature: &TileSignature) -> Result<&TileIdentity, Error>
  {
    self.limiter.rearrange(signature);
    return match self.table.get(signature) {
      None => Err(Error::NoSuchTile(signature.clone())),
      Some(x) => Ok(x.as_ref())
    }
  }

  pub fn has(&self, signature: TileSignature) -> bool
  {
    return self.table.contains_key(&signature);
  }

  pub fn load(&mut self, signature: &TileSignature) -> Result<&TileIdentity, Error>
  {
    if self.network.is_unavailable(signature) {
      return Err(Error::NoSuchObjectInRemote(signature.clone()))
    }

    return match self.cache(signature) {
      Ok(_) => self.get(signature),
      Err(_) => {
        self.download(signature)?;
        self.get(signature)
      }
    }
  }

  fn unload(&mut self, signature: &TileSignature) -> Result<(), Error>
  {
    println!("Unloading {}", signature);
    return match self.table.remove(signature) {
      None => Err(Error::NoSuchTile(signature.clone())),
      Some(_) => Ok(())
    }
  }

  fn cache(&mut self, signature: &TileSignature) -> Result<(), Error>
  {
    if !self.is_cached(signature) {
      return Err(Error::NoSuchTile(signature.clone()))
    }

    self.add(signature)?;
    Ok(())
  }

  fn download(&mut self, signature: &TileSignature) -> Result<(), Error>
  {
    return match self.network.download_tile(signature) {
      Ok(_) => self.cache(signature),
      Err(e) => {
        self.network.make_unavailable(signature);
        Err(e)
      }
    };
  }

  fn is_cached(&self, signature: &TileSignature) -> bool
  {
    return std::path::Path::new(&signature.to_abs_path())
      .exists()
  }

  fn add(&mut self, signature: &TileSignature) -> Result<(), Error>
  {
    match self.limiter.add(signature) {
      None => (),
      Some(x) => self.unload(&x)?
    };
    self.table.insert(*signature, Box::new(
      match TileIdentity::new(signature.to_abs_path()) {
        Ok(x) => x,
        Err(e) => return Err(e)
      }
    ));
    Ok(())
  }
}


