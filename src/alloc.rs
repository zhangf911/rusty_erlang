use world;

#[allow(dead_code)]
enum AllocatorStrategyType {
  GoodFit,
  BestFit,
  AFit,
  AOFirstFit,
}

pub fn init(_args: &Vec<String>, _state: &mut world::Erts) {
}
