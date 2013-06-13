use narrow::collision_detector::CollisionDetector;

/// Collision detector working with any geometry. It will never report any
/// collision. Used for debug purpoise.
/// Use it if you want to benchmark the broad phase.
pub struct EmptyDetector;

impl<C, G1, G2> CollisionDetector<C, G1, G2> for EmptyDetector
{
  fn new(_: &G1, _: &G2) -> EmptyDetector
  { EmptyDetector }

  fn update(&mut self, _: &G1, _: &G2)
  { }

  fn num_coll(&self) -> uint
  { 0u }

  fn colls(&mut self, _: &mut ~[@mut C])
  { }
}