use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MapTile
{
  pub zoom: i32,
  pub x: i32,
  pub y: i32
}

impl Display for MapTile
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "({}, {}, zoom: {})", self.x, self.y, self.zoom)
  }
}

impl MapTile
{
  pub fn new(zoom: i32, x: i32, y: i32) -> MapTile { MapTile { zoom, x, y } }
}