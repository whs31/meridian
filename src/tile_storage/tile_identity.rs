pub struct TileIdentity
{
  pub file_path: String
  // tiff data
}

impl TileIdentity
{
  pub fn new(file_path: String) -> Self
  {
    Self {
      file_path
    }
  }
}