use futures_util::stream::StreamExt;
use std::collections::HashSet;
use std::io::Write;
use std::path::MAIN_SEPARATOR;
use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use crate::errors::Error;
use crate::tile_storage::TileSignature;

pub struct NetworkFetcher
{
  pub client: reqwest::Client,
  unavailable: HashSet<TileSignature>
}

impl NetworkFetcher
{
  pub fn new() -> Self
  {
    Self
    {
      client: reqwest::Client::new(),
      unavailable: HashSet::new()
    }
  }

  pub fn is_unavailable(&self, signature: &TileSignature) -> bool
  {
    return self.unavailable.contains(signature);
  }

  pub fn make_unavailable(&mut self, signature: &TileSignature)
  {
    self.unavailable.insert(signature.clone());
  }

  #[tokio::main]
  pub async fn download_tile(&mut self, signature: &TileSignature) -> Result<(), Error>
  {
    if self.is_unavailable(signature) {
      return Err(Error::NoSuchObjectInRemote(signature.clone()));
    }

    let start = Utc::now().time();

    let target = signature.to_abs_path();
    let source = signature.to_url();
    debug!("Downloading file {} from {}", target, source);
    let response = self.client
      .get(&source)
      .send()
      .await?;

    if !response.status().is_success() {
      return Err(Error::NetworkStatusCodeError(response.status().as_u16(), signature.clone()));
    }

    let total = response
      .content_length()
      .unwrap_or(1);
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::with_template(
      "{wide_msg} {spinner:.green} [{bar:20.yellow/white}] \
      {bytes:10}/ {total_bytes:10} ({percent:3}%)",)
      .unwrap()
      .progress_chars("█░░"));
    pb.set_message(format!("Downloading {} from {}", signature, source));

    debug!("Making missing folders to target {target}...");
    std::fs::create_dir_all(target[..target.rfind(MAIN_SEPARATOR).unwrap()]
      .to_string())
      .unwrap();
    let mut file = std::fs::File::create(&target)?;
    debug!("File status: OK");

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
      let chunk = item.unwrap();
      file.write_all(&chunk)?;
      let new = (downloaded + chunk.len() as u64).min(total);
      downloaded = new;
      pb.set_position(new);
    }
    pb.finish_with_message(format!("Downloaded {} to {}", signature, target));
    println!();

    debug!("File successfully downloaded to {}", target);

    let end = Utc::now().time();
    let duration = (end - start).num_milliseconds();
    info!("Tile {:?} downloaded in {}ms", signature, duration);
    Ok(())
  }
}