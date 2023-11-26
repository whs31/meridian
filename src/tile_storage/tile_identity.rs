use log::{debug};
use crate::errors::Error;
use chrono::Utc;
use crate::geotiff::GeoTiff;

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
    let data_raw = GeoTiff::from_file(&file_path)?;
    let end = Utc::now().time();
    debug!("Decoding status: OK");
    debug!("Decoding tiff file from {} took {}ms", file_path, (end - start).num_milliseconds());

    let im_size = imagesize::size(&file_path)?;
    debug!("Image size: {:?}", im_size);

    Ok(Self {
      file_path,
      data: Box::new(data_raw),
      size: (im_size.width, im_size.height)
    })
  }
}