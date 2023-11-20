use futures_util::stream::StreamExt;
use std::env::current_dir;
use std::fs;
use std::io::{Write};
use std::path::MAIN_SEPARATOR;
use std::sync::Mutex;
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use crate::config::CONFIG;
use crate::errors::Error;
use crate::tile_storage::tile_identity::TileIdentity;
use crate::tile_storage::tile_signature::TileSignature;
use crate::utils::StaticHeapObject;

pub static STORAGE: StaticHeapObject<TileStorage> = Lazy::new(
  || { Mutex::new(Box::new(TileStorage::new())) }
);

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

  pub fn is_cached(&self, signature: TileSignature) -> bool
  {
    let path = self.get_absolute_path(&signature);
    debug!("Checking if signature {:?} at path {} exists", signature, path);
    return std::path::Path::new(&path).exists();
  }

  pub fn get_or_emplace(&mut self, signature: TileSignature) -> Result<&TileIdentity, Error>
  {
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

    return match self.download_tile(&signature) {
      Ok(_) => { self.get(signature) },
      Err(e) => {
        error!("No such tile in remote url. Please check if the tile is available: {:?} [{:?}]",
          signature, e);
        Err(e)
      }
    };
  }

  #[tokio::main]
  async fn download_tile(&mut self, signature: &TileSignature) -> Result<(), Error>
  {
    let start = Utc::now().time();

    let target = self.get_absolute_path(signature);
    let source = self.get_url(signature);
    debug!("Downloading file {} from {}", target, source);
    let response = match reqwest::get(&source).await {
      Ok(x) => x,
      Err(_) => { return Err(Error::NetworkFailure) }
    };
    let total = response
      .content_length()
      .unwrap_or(1);
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::with_template(
      "{wide_msg} {spinner:.green} [{bar:20.yellow/white}] \
      {bytes:10}/ {total_bytes:10} ({percent:3}%)",)
      .unwrap()
      .progress_chars("█░░"));
    pb.set_message(format!("Downloading {:?} from {}", signature, source));

    debug!("Checking if response is successful...");
    if !response.status().is_success() {
      warn!("Response status: ERROR");
      return Err(Error::NetworkFailure);
    }
    debug!("Response status: OK");
    debug!("Making missing folders to target {target}...");
    fs::create_dir_all(target[..target.rfind(MAIN_SEPARATOR).unwrap()].to_string()).unwrap();
    let mut file = match fs::File::create(&target) {
      Ok(x) => { x }
      Err(_) => { return Err(Error::FileCreationFailure) }
    };
    debug!("File status: OK");

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
      let chunk = item.unwrap();
      match file.write_all(&chunk).or(Err(Error::WriteToFileFailure)) {
        Ok(_) => {},
        Err(_) => {
          warn!("Failed to download or emplace file in cache at: {}, from: {}", target, source);
          return Err(Error::WriteToFileFailure);
        }
      }
      let new = (downloaded + chunk.len() as u64).min(total);
      downloaded = new;
      pb.set_position(new);
    }
    pb.finish_with_message(format!("Downloaded {:?} to {}", signature, target));
    println!();

    debug!("File successfully downloaded to {}", target);
    debug!("Checking if signature {:?} is cached...", signature);
    if self.is_cached(signature.clone()) {
      debug!("Tile {:?} is cached, loading...", signature);
      self.insert(signature)?;
    }

    let end = Utc::now().time();
    let duration = (end - start).num_milliseconds();
    info!("Tile {:?} downloaded and cached in {}ms", signature, duration);
    Ok(())
  }

  fn insert(&mut self, signature: &TileSignature) -> Result<(), Error>
  {
    self.table.insert(*signature, Box::new(match TileIdentity::new(
      self.get_absolute_path(&signature)
    ) {
      Ok(x) => { x },
      Err(e) => {
        error!("Failed to decode tiff file from {}", self.get_absolute_path(&signature));
        return Err(e);
      }
    }));
    Ok(())
  }

  fn get_absolute_path(&self, signature: &TileSignature) -> String
  {
    format!("{}{MAIN_SEPARATOR}{}{MAIN_SEPARATOR}{}",
      current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap(),
      CONFIG
        .lock()
        .unwrap()
        .get("Elevation", "cache_dir")
        .unwrap(), signature.to_relative_path())
  }

  fn get_url(&self, signature: &TileSignature) -> String
  {
    format!("{}/{}", CONFIG
      .lock()
      .unwrap()
      .get("Elevation", "remote_url")
      .unwrap(), signature.to_relative_path()
      .replace('\\', "/"))
  }
}