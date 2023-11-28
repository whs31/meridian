use std::collections::HashSet;
use std::io::Write;
use std::path::MAIN_SEPARATOR;
use futures::{stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, warn};
use meridian_positioning::errors::PositioningError;
use meridian_positioning::GeoRectangle;
use crate::errors::Error;
use crate::tile_storage::TileSignature;

#[derive(Debug)]
pub struct ElevationPrefetcher
{
  client: reqwest::Client,
  server_url: String,
  storage_url: String,
  extension: String,
  parallel_threads: usize,
  unavailable: HashSet<TileSignature>
}

impl ElevationPrefetcher
{
  pub fn new(server_url: String, storage_url: String, extension: String, parallel_threads: usize)
    -> ElevationPrefetcher
  {
    ElevationPrefetcher
    {
      client: reqwest::Client::new(),
      server_url,
      storage_url,
      extension,
      parallel_threads,
      unavailable: HashSet::new()
    }
  }

  #[tokio::main]
  pub async fn fetch(&mut self, rect: GeoRectangle) -> Result<(), Error>
  {
    let signatures = ElevationPrefetcher::split_rectangle(rect)?
      .iter()
      .cloned()
      .filter(|s| !self.unavailable.contains(s) && !self.is_cached(
        s.to_abs_path_threadsafe(self.extension.as_str(), self.storage_url.as_str())
          .as_str()
      ))
      .collect::<Vec<TileSignature>>();

    let progress_bar = ProgressBar::new(signatures.len() as u64);
    progress_bar.set_style(ProgressStyle::with_template(
      "{wide_msg} {spinner:.green} [{bar:20.yellow/white}] {pos:10}/ {len:10} ({percent:3}%)")
      .unwrap()
      .progress_chars("█░░"));
    progress_bar.set_message(format!("Downloading elevation tiles from {}", self.server_url.as_str()));
    let bodies = stream::iter(signatures)
      .enumerate()
      .map(|(i, signature)| {
        let url = signature.to_url_threadsafe(self.extension.as_str(), self.server_url.as_str());
        let fs_path = signature.to_abs_path_threadsafe(self.extension.as_str(), self.storage_url.as_str());

        let client = self.client.clone();
        tokio::spawn(async move {
          let response = client.get(&url).send().await?;
          if !response.status().is_success() {
            return Err(Error::NetworkStatusCodeErrorStr(response.status().as_u16(), url.clone()));
          }

          std::fs::create_dir_all(fs_path[..fs_path.rfind(MAIN_SEPARATOR).unwrap()]
            .to_string()
          )?;
          let mut file = std::fs::File::create(&fs_path)?;
          let mut stream = response.bytes_stream();
          while let Some(item) = stream.next().await
          {
            let chunk = item?;
            file.write_all(&chunk)?;
          }
          Ok(())
        })
      }).buffer_unordered(self.parallel_threads);

    bodies
      .enumerate()
      .for_each(|(i, b)| async {
        progress_bar.inc(1);
        match b {
          Err(e) => error!("Got a tokio::JoinError: {}", e),
          _ => ()
        }
      })
      .await;
    progress_bar.finish_with_message("Elevation tiles downloaded");
    println!();
    Ok(())
  }

  fn is_cached(&self, path: &str) -> bool { std::path::Path::new(path).exists() }

  fn split_rectangle(rect: GeoRectangle) -> Result<Vec<TileSignature>, Error>
  {
    if !rect.valid() { return Err(Error::Positioning(PositioningError::InvalidGeorectangle(rect.clone()))) }

    let left = rect.top_left().longitude.floor() as i16;
    let right = rect.bottom_right().longitude.floor() as i16;
    let top = rect.top_left().latitude.floor() as i8;
    let bottom = rect.bottom_right().latitude.floor() as i8;

    let signatures: Vec<TileSignature> = (bottom..=top)
      .flat_map(|lat| (left..=right)
        .map(move |lon| TileSignature::new(lat, lon)))
      .collect();

    Ok((signatures))
  }
}