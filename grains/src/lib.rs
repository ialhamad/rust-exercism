pub fn square(s: u32) -> u64 {
  if s > 0 && s <= 64 {
    1u64 << s - 1
  } else {
    panic!("Square must be between 1 and 64");
  }
}

pub fn total() -> u64 {
  ((1u128 << 64) - 1) as u64
}
