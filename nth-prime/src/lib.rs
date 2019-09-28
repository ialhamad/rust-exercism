pub fn nth(n: u32) -> u32 {
  let mut nth_prime: u32 = 2;
  let mut i = 3;
  let mut c = 1;
  while c <= n {
    for j in 2..=i {
      if j == i {
        nth_prime = i;
        c += 1;
      }
      if i % j == 0 {
        break;
      }
    }
    i += 1;
  }
  nth_prime
}
