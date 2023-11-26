use meridian_positioning::GeoCoordinate;
use crate::errors::Error;
use crate::tile_storage::TileSignature;
use crate::tile_storage::STORAGE;
use crate::utils::validate_coordinate;

pub trait Elevation {
  fn elevation(&self) -> Result<f32, Error>;
}

impl Elevation for GeoCoordinate
{
  fn elevation(&self) -> Result<f32, Error>
  {
    elevation_at((self.latitude, self.longitude))
  }
}

pub fn elevation_at(coordinate: (f64, f64)) -> Result<f32, Error>
{
  let mut storage = STORAGE
    .lock()
    .unwrap();
  let coord = validate_coordinate(coordinate)?;
  let key = TileSignature::from_f64(coord.0, coord.1);
  let val = match storage.get(&key) {
    Ok(x) => x,
    Err(_) => storage.load(&key)?
  };
  let image_size = val.size;
  let data = val.data.as_ref();
  let tile_size = key.georectangle_size();
  let requested_coordinate = GeoCoordinate::new(coord.0, coord.1, None);
  let distance_2d = (
    requested_coordinate.distance_to(&GeoCoordinate::new(coord.0, key.longitude as f64, None))?,
    requested_coordinate.distance_to(&GeoCoordinate::new(key.latitude as f64, coord.1, None))?
  );
  let dn = (distance_2d.0 / (tile_size.1 as f32), distance_2d.1 / (tile_size.0 as f32));
  let pixel_coords = ((dn.0 * image_size.0 as f32) as usize,
                                      (dn.1 * image_size.1 as f32) as usize);
  let value = data.get_pixel(pixel_coords.0, pixel_coords.1);

  Ok(value as f32)
}