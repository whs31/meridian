use crate::errors::Error;

pub struct ElevationPoint
{
  pub distance: f32,
  pub elevation: f32
}



// pub fn build_profile(path: Vec<GeoCoordinate>, discrete: u8) -> Result<Vec<ElevationPoint>, Error>
// {
//   if path.is_empty() {
//     return Err(Error::EmptyPath);
//   }
//   let mut profile: Vec<ElevationPoint> = Vec::new();
//   let distance_from_start = 0.0;
//   let prev_base_point_geo = path.get(0).unwrap();
//   for point in path {
//     if !profile.is_empty() {
//       let azimuth = prev_base_point_geo.to_wgs84().
//     }
//   }
//   Ok()
// }