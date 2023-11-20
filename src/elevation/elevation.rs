use crate::errors::Error;
use crate::positioning::geocoordinate::GeoCoordinate;
use crate::tile_storage::tile_signature::TileSignature;
use crate::tile_storage::tile_storage::STORAGE;
use crate::utils::validate_coordinate;

pub trait Elevation {
  fn elevation(&self) -> Result<f32, Error>;
}

pub fn elevation_at_coordinate(coordinate: &GeoCoordinate) -> Result<f32, Error>
{
  elevation_at((coordinate.latitude, coordinate.longitude))
}
pub fn elevation_at(coordinate: (f64, f64)) -> Result<f32, Error>
{
  let mut storage = STORAGE
    .lock()
    .unwrap();
  let coord = validate_coordinate(coordinate)?;
  let key = TileSignature::from_f64(coord.0, coord.1);
  let image_size = storage.get_or_emplace(key)?.size;
  let data = &storage.get_or_emplace(key)?.data;
  let tile_size = key.georectangle_size();
  let requested_coordinate = GeoCoordinate::new(coord.0, coord.1, 0.0);
  let distance_2d = (
    requested_coordinate.distance_to(&GeoCoordinate::new(coord.0, key.longitude as f64, 0.0))?,
    requested_coordinate.distance_to(&GeoCoordinate::new(key.latitude as f64, coord.1, 0.0))?
  );
  let dn = (distance_2d.0 / (tile_size.1 as f32), distance_2d.1 / (tile_size.0 as f32));
  let pixel_coords = ((dn.0 * image_size.0 as f32) as usize, (dn.1 * image_size.1 as f32) as usize);
  let value = data.get_pixel(pixel_coords.0, pixel_coords.1);

  Ok(value as f32)
}