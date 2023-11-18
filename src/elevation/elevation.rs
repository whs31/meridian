use log::debug;
use nav_types::WGS84;
use crate::errors::Error;
use crate::tile_storage::tile_signature::TileSignature;
use crate::tile_storage::tile_storage::STORAGE;
use crate::utils::validate_coordinate;

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
  let requested_coordinate = WGS84::from_degrees_and_meters(coord.0,
                                                            coord.1, 0.0);
  let distance_2d = (requested_coordinate.distance(&WGS84::from_degrees_and_meters(coord.0,
                                                                                   key.longitude
                                                                                     as f64, 0.0)),
                                 requested_coordinate.distance(&WGS84::from_degrees_and_meters(key.latitude
                                                                                     as f64,
                                                                                               coord.1,
                                                                                   0.0)));
  let dn = (distance_2d.0 / (tile_size.1 as f64), distance_2d.1 / (tile_size.0 as f64));
  let pixel_coords = ((dn.0 * image_size.0 as f64) as usize, (dn.1 * image_size.1 as f64) as usize);
  debug!("Pixel coords: {:?}", pixel_coords);
  let value = data.get_pixel(pixel_coords.1, pixel_coords.0);
  debug!("Elevation at {:?} is {} meters", coord, value);

  Ok(value as f32)
}