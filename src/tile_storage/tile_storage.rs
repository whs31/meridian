use std::collections::{HashMap, VecDeque};
use std::io::sink;
use std::sync::Mutex;
use log::{debug, info, warn};
use once_cell::sync::Lazy;
use reqwest::get;
use crate::errors::Error;
use crate::tile_storage::net_fetch::NetworkFetcher;
use crate::tile_storage::tile_identity::TileIdentity;
use crate::tile_storage::tile_signature::TileSignature;
use crate::utils::StaticHeapObject;

pub static STORAGE: StaticHeapObject<TileStorage> = Lazy::new(
  || { Mutex::new(Box::new(TileStorage::new())) }
);

pub struct TileStorage
{
  table: HashMap<TileSignature, Box<TileIdentity>>,
  network: NetworkFetcher,
  limiter: MemoryLimiter
}

struct MemoryLimiter
{
  pub max_tile_count: usize,
  queue: VecDeque<TileSignature>
}

impl MemoryLimiter
{
  pub fn new(max_tile_count: usize) -> Self
  {
    Self {
      max_tile_count,
      queue: VecDeque::new()
    }
  }

  pub fn test(&mut self, signature: &TileSignature) -> Option<TileSignature>
  {
    return match self.queue
      .iter()
      .position(|x| x == signature) {
      None => {
        self.queue.push_front(signature.clone());
        if self.queue.len() > self.max_tile_count {
          self.queue.pop_back();
          Some(signature.clone())
        }
        else { None }
      }
      Some(x) => {
        self.queue.remove(x);
        self.queue.push_front(signature.clone());
        None
      }
    }
  }
}

impl TileStorage
{
  pub fn new() -> Self
  {
    Self {
      table: HashMap::new(),
      network: NetworkFetcher::new(),
      limiter: MemoryLimiter::new(5)
    }
  }

  pub fn get(&self, signature: &TileSignature) -> Result<&TileIdentity, Error>
  {
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
    self.table.insert(*signature, Box::new(
      match TileIdentity::new(signature.to_abs_path()) {
        Ok(x) => x,
        Err(e) => return Err(e)
      }
    ));
    Ok(())
  }
}


