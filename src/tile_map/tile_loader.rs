use std::collections::VecDeque;
use std::f64::consts::PI;
use std::ops::{Div, Mul, Range};
use meridian_positioning::GeoRectangle;
use crate::coordinate_system::{OSM_MAX_ZOOM, OSM_MIN_ZOOM};
use crate::errors::Error;
use crate::tile_map::MapTile;

const MAX_ALLOWED_PARALLEL: usize = 4;

struct TileLoader
{
  pub server_url: String,
  pub storage_url: String,
  client: reqwest::Client,
  total_tiles: usize,
  loaded_tiles: usize,
  parallel_loaded_count: usize,
  queue: VecDeque<MapTile>
}

impl TileLoader
{
  pub fn new(server_url: String, storage_url: String) -> TileLoader
  {
    TileLoader
    {
      server_url,
      storage_url,
      client: reqwest::Client::new(),
      total_tiles: 0,
      loaded_tiles: 0,
      parallel_loaded_count: 0,
      queue: VecDeque::new()
    }
  }

  pub async fn download(&mut self, zoom: i32, x: i32, y: i32) -> Result<(), Error>
  {
    self.queue.push_back(MapTile::new(zoom, x, y));
    self.total_tiles += 1;
    if self.parallel_loaded_count < MAX_ALLOWED_PARALLEL {
      self.process_queue().await?;
    }
    Ok(())
  }

  pub async fn download_georectangle(&mut self, zoom_range: Range<i32>, rect: GeoRectangle)
    -> Result<(), Error>
  {
    // if zoom_range.start < OSM_MIN_ZOOM as i32 || zoom_range.end > OSM_MAX_ZOOM as i32 {
    //   return Err(Error::InvalidArgument(format!("Zoom={:?} must be in range {}-{}",
    //                                             zoom_range,
    //                                             OSM_MIN_ZOOM,
    //                                             OSM_MAX_ZOOM)));
    // }
    //
    // for z in zoom_range {
    //
    // }
    //
    Ok(())
  }

  async fn process_queue(&mut self) -> Result<(), Error>
  {
    todo!("Implement me")
  }

  fn lat_to_tile(lat: f64, zoom: u8) -> u32
  {
    (1.0 - lat
      .to_radians()
      .tan()
      .asinh()
      .div(PI)
      .div(2.0)
      .mul((1 << zoom) as f64)
    ).floor() as u32
  }
  fn lon_to_tile(lon: f64, zoom: u8) -> u32 { ((lon + 180.0) / 360.0 * (1 << zoom) as f64).floor() as u32 }
  fn tile_to_lat(y: u32, zoom: u8) -> f64
  {
    let n = PI * (1.0 - 2.0 * y as f64 / (1 << zoom) as f64);
    (n.exp() - (-n).exp())
      .mul(0.5)
      .atan()
      .to_degrees()
  }
  fn tile_to_lon(x: u32, zoom: u8) -> f64 { x as f64 / (1 << zoom) as f64 * 360.0 - 180.0 }
}

// http://uav.radar-mms.com/gitlab/test/qtex/qtex/-/blob/main/libs/geo/src/c%2B%2B/qtexgeo-tileloader.c%2B%2B