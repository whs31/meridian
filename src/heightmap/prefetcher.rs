use std::collections::HashSet;
use futures::{stream, StreamExt};
use meridian_positioning::errors::PositioningError;
use meridian_positioning::GeoRectangle;
use crate::config::CONFIG;
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
    let sig_raw = ElevationPrefetcher::split_rectangle(rect)?;
    let signatures = sig_raw
      .iter()
      .filter(|s| !self.unavailable.contains(s))
      .collect::<Vec<_>>();
    let urls = signatures
      .iter()
      .map(|s| s.to_url_threadsafe( self.extension.as_str(), self.server_url.as_str()))
      .collect::<Vec<_>>();

    let bodies = stream::iter(urls)
      .map(|url| {
        // println!("{}", url);
        let client = self.client.clone();
        tokio::spawn(async move {
          let response = client.get(url).send().await?;
          response.bytes().await
        })
      }).buffer_unordered(self.parallel_threads);

    bodies.for_each(|b| async {
      match b {
        Ok(Ok(b)) => println!("Got {} bytes", b.len()),
        Ok(Err(e)) => eprintln!("Got error: {}", e),
        Err(e) => eprintln!("Got error(tokio): {}", e)
      }
    }).await;
    Ok(())
  }

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