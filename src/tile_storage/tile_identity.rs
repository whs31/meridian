use geotiff_rs::{GeoTiff};
use log::{debug, warn};
use crate::errors::Error;
use chrono::Utc;

pub struct TileIdentity
{
  pub file_path: String,
  pub data: Box<GeoTiff>,
  pub size: (usize, usize)
}

impl TileIdentity
{
  pub fn new(file_path: String) -> Result<Self, Error>
  {
    debug!("Decoding tiff file from {}", file_path);
    let start = Utc::now().time();
    let data_raw = match GeoTiff::from_file(&file_path) {
      Ok(x) => { x },
      Err(_) => {
        warn!("Failed to decode tiff file from {}", file_path);
        return Err(Error::TiffError);
      }
    };
    let end = Utc::now().time();
    debug!("Decoding status: OK");
    debug!("Decoding tiff file from {} took {}ms", file_path, (end - start).num_milliseconds());

    let im_size = match imagesize::size(&file_path) {
      Ok(x) => { x },
      Err(_) => {
        warn!("Failed to get image size from {}", file_path);
        return Err(Error::ImageSizeError);
      }
    };
    debug!("Image size: {:?}", im_size);

    Ok(Self {
      file_path,
      data: Box::new(data_raw),
      size: (im_size.width, im_size.height)
    })
  }
}