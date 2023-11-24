// use std::collections::VecDeque;
// use crate::tile_storage::tile_signature::TileSignature;
//
// pub struct TileLimiter
// {
//   pub max_count: usize,
//   priority_queue: VecDeque<TileSignature>
// }
//
// impl TileLimiter
// {
//   pub fn new(max_count: usize) -> Self
//   {
//     Self
//     {
//       max_count,
//       priority_queue: VecDeque::new()
//     }
//   }
//
//   pub fn rearrange(&mut self, signature: &TileSignature)
//   {
//     match self.priority_queue
//       .iter()
//       .position(|x| x == signature)
//       {
//         None => {}
//         Some(x) => {
//           self.priority_queue.remove(x);
//           self.priority_queue.push_front(signature.clone());
//         }
//       }
//   }
// }