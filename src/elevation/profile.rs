// use crate::elevation_at;
// use crate::errors::Error;
// use crate::positioning::geocoordinate::GeoCoordinate;
//
// pub struct ElevationPoint
// {
//   pub distance: f32,
//   pub elevation: f32
// }

// pub fn build_profile(path: Vec<GeoCoordinate>, discrete: u8) -> Result<Vec<ElevationPoint>, Error>
// {
//   if path.is_empty() {
//     return Err(Error::EmptyPath);
//   }
//   let mut profile: Vec<ElevationPoint> = Vec::new();
//   let mut distance_from_start = 0.0;
//   let mut prev_base_point_geo = path.get(0).unwrap();
//   for mut point in path {
//     if !profile.is_empty() {
//       let azimuth = prev_base_point_geo.azimuth_to(&point)?;
//       let distance_from_prev_base_point = prev_base_point_geo.distance_to(&point)?;
//       let mut distance = discrete as f32;
//       let mut prev_delta_point_geo = prev_base_point_geo.clone();
//       while distance < distance_from_prev_base_point {
//         let mut delta_point_geo = prev_base_point_geo
//           .at_distance_and_azimuth(distance, azimuth, 0.0)?;
//         let new_altitude = elevation_at((delta_point_geo.latitude, delta_point_geo.longitude))?;
//         delta_point_geo.altitude = new_altitude as f64;
//
//         if prev_delta_point_geo.altitude != delta_point_geo.altitude {
//           if prev_delta_point_geo.altitude > delta_point_geo.altitude {
//             profile.push(ElevationPoint {
//               distance: distance - discrete as f32 + distance_from_start,
//               elevation: prev_delta_point_geo.altitude as f32
//             });
//           }
//           else {
//             profile.push(ElevationPoint {
//               distance: distance + distance_from_start,
//               elevation: delta_point_geo.altitude as f32
//             });
//           }
//         }
//         prev_delta_point_geo = &delta_point_geo;
//         distance += discrete as f32;
//       }
//       distance_from_start += distance_from_prev_base_point;
//     }
//     let new_altitude = elevation_at((point.latitude, point.longitude))?;
//     point.altitude = new_altitude as f64;
//     profile.push(ElevationPoint {
//       distance: distance_from_start,
//       elevation: point.altitude as f32
//     });
//     prev_base_point_geo = &point;
//   }
//
//   Ok(profile)
// }
//
// pub fn build_profile_as_geopath(path: Vec<GeoCoordinate>, step: f32) -> Result<Vec<GeoCoordinate>, Error>
// {
//   if path.is_empty() {
//     return Err(Error::EmptyPath);
//   }
//   let mut profile: Vec<GeoCoordinate> = Vec::new();
//   for mut point in path {
//     if !profile.is_empty() {
//       let previous = profile.last().unwrap();
//       let azimuth = previous.azimuth_to(&point)?;
//       let delta = previous.distance_to(&point)?;
//       let delta2 = step;
//       let t = previous.clone();
//       while delta2 < delta {
//         let u = previous.at_distance_and_azimuth(delta2, azimuth, 0.0)?;
//         let alt = elevation_at((u.latitude, u.longitude))?;
//         u.setAltitude(alt as f64);
//         if t.altitude() != u.altitude() {
//           if t.altitude() > u.altitude() {
//             profile.push(t);
//           }
//         }
//       }
//     }
//   }
//   for(auto point : path.path())
//   {
//     if(ret.size())
//     {
//       auto previous = ret.coordinateAt(ret.size() - 1);
//       auto azimuth = previous.azimuthTo(point);
//       auto delta = previous.distanceTo(point);
//       f32 delta2 = step;
//       auto t = previous;
//       while(delta2 < delta)
//       {
//         auto u = previous.atDistanceAndAzimuth(delta2, azimuth);
//         auto alt = elevation(u);
//         if(not alt.has_value())
//         return ret;
//         u.setAltitude(alt.value());
//         if(t.altitude() != u.altitude())
//         {
//           if(t.altitude() > u.altitude()) ret.addCoordinate(t);
//           else ret.addCoordinate(u);
//         }
//         t = u;
//         delta2 += step;
//       }
//     }
//     auto alt = elevation(point);
//     if(not alt.has_value())
//     return ret;
//     point.setAltitude(alt.value());
//     ret.addCoordinate(point);
//   }
//   return ret;
// }
//
// }