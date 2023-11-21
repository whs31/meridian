use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use futures_util::future::err;
use log::{debug, error, info, warn};
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
  deque: VecDeque<TileSignature>,
  network: NetworkFetcher
}

impl TileStorage
{
  pub fn new() -> Self
  {
    Self {
      table: HashMap::new(),
      deque: VecDeque::new(),
      network: NetworkFetcher::new()
    }
  }

  pub fn is_available(&self, signature: TileSignature) -> bool
  {
    return self.table.contains_key(&signature);
  }

  pub fn get(&self, signature: TileSignature) -> Result<&TileIdentity, Error>
  {
    return match self.table.get(&signature) {
      None => { Err(Error::NoSuchTile(signature)) },
      Some(x) => { Ok(x.as_ref()) }
    }
  }

  pub fn is_cached(&self, signature: TileSignature) -> bool
  {
    let path = &signature.to_abs_path();
    debug!("Checking if signature {:?} at path {} exists", signature, path);
    return std::path::Path::new(&path).exists();
  }

  pub fn get_or_emplace(&mut self, signature: TileSignature) -> Result<&TileIdentity, Error>
  {
    if self.network.is_unavailable(&signature) {
      return Err(Error::NoSuchObjectInRemote(signature.clone()));
    }

    if self.is_available(signature.clone()) {
      return self.get(signature);
    }
    debug!("Checking if signature {:?} is cached...", signature);
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
  fn filter(&mut self, signature: &TileSignature)
  {
    let pos = match self.deque.binary_search(signature) {
      Ok(x) => { x }
      Err(_) => {
        self.deque.push_back(*signature);
        self.deque.len() - 1
      }
    };
    self.deque.swap(0, pos);
    if self.deque.len() > 20 {
      match self.deque.pop_back() {
        None => {}
        Some(sign) => { self.table.remove(&sign); }
      }
    }
  }
}


