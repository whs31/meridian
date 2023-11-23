use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use log::{debug, info, warn};
use once_cell::sync::Lazy;
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

  pub fn is_available(&self, signature: TileSignature) -> bool
  {
    return self.table.contains_key(&signature);
  }

  pub fn test(&mut self, signature: &TileSignature)
  {
    match self.limiter.test(&signature) {
      Some(x) => {
        let rm = self.table.remove(&x);
        println!("{}, {}", self.table.len(), rm.is_none());
      },
      None => {}
    }
  }

  pub fn get(&mut self, signature: TileSignature) -> Result<&TileIdentity, Error>
  {
    match self.table.get(&signature) {
      None => { Err(Error::NoSuchTile(signature)) },
      Some(x) => {
        self.test(&signature);
        Ok(x.as_ref())
      }
    }
  }

  pub fn is_cached(&self, signature: TileSignature) -> bool
  {
    let path = &signature.to_abs_path();
    return std::path::Path::new(&path).exists();
  }

  pub fn emplace(&mut self, signature: TileSignature) -> Result<&TileIdentity, Error>
  {
    if self.network.is_unavailable(&signature) {
      return Err(Error::NoSuchObjectInRemote(signature.clone()));
    }

    if self.is_available(signature.clone()) {
      return self.get(signature);
    }

    if self.is_cached(signature.clone()) {
      debug!("Tile {:?} is cached, loading...", signature);
      self.insert(&signature)?;
      return self.get(signature);
    }
    info!("Tile {:?} is not available, downloading...", signature);

    return match self.network.download_tile(&signature) {
      Ok(_) => {
        if self.is_cached(signature.clone()) {
          debug!("Tile {:?} is cached, loading...", &signature);
          self.insert(&signature)?;
        }
        self.get(signature)
      },
      Err(e) => {
        self.network.make_unavailable(&signature);
        match e {
          Error::NoSuchObjectInRemote(_) => Err(e),
          _ => {
            warn!("{}", e);
            Err(e)
          }
        }
      }
    };
  }

  fn insert(&mut self, signature: &TileSignature) -> Result<(), Error>
  {
    self.table.insert(*signature, Box::new(
      match TileIdentity::new(signature.to_abs_path()) {
        Ok(x) => { x },
        Err(e) => return Err(e)
      }
    ));
    Ok(())
  }
}


