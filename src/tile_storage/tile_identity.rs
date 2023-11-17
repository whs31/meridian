use geotiff_rs::{GeoTiff};
use log::{debug, warn};
use crate::errors::Error;

pub struct TileIdentity
{
  pub file_path: String,
  pub data: Box<GeoTiff>
}

impl TileIdentity
{
  pub fn new(file_path: String) -> Result<Self, Error>
  {
    debug!("Decoding tiff file from {}", file_path);
    let data_raw = match GeoTiff::from_file(&file_path) {
      Ok(x) => { x },
      Err(_) => {
        warn!("Failed to decode tiff file from {}", file_path);
        return Err(Error::TiffError);
      }
    };

    debug!("Decoding status: OK");
    Ok(Self {
      file_path,
      data: Box::new(data_raw)
    })
  }
}