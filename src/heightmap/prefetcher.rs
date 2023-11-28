use meridian_positioning::errors::PositioningError;
use meridian_positioning::GeoRectangle;
use parallel_downloader::SourceTarget;
use crate::errors::Error;
use crate::tile_storage::TileSignature;

#[derive(Debug)]
pub struct ElevationPrefetcher
{
  server_url: String,
  storage_url: String,
  extension: String,
  parallel_threads: usize
}

impl ElevationPrefetcher
{
  pub fn new(server_url: String, storage_url: String, extension: String, parallel_threads: usize)
    -> ElevationPrefetcher
  {
    ElevationPrefetcher
    {
      server_url,
      storage_url,
      extension,
      parallel_threads
    }
  }

  #[tokio::main]
  pub async fn fetch(&mut self, rect: GeoRectangle) -> Result<(), Error>
  {
    let signatures = ElevationPrefetcher::split_rectangle(rect)?
      .iter()
      .cloned()
      .filter(|s| !self.is_cached(
        s.to_abs_path_threadsafe(self.extension.as_str(), self.storage_url.as_str())
          .as_str()
      ))
      .collect::<Vec<TileSignature>>();

    let source_list: Vec<SourceTarget> = signatures
      .iter()
      .map(|s| {
        let b = s.to_abs_path_threadsafe(
          self.extension.as_str(),
          self.storage_url.as_str()
        );
        let target = std::path::Path::new(b.as_str())
          .parent()
          .unwrap()
          .to_str()
          .unwrap();
        SourceTarget
        {
          source: s.to_url_threadsafe(
            self.extension.as_str(),
            self.server_url.as_str()
          ),
          target: target.to_string()
        }
      })
      .collect();

    parallel_downloader::parallel_download(
      source_list,
      self.parallel_threads,
      1,
      10
    ).await?;
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