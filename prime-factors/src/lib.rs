pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut p = 2;
    while n > 1 {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
        p += 1;
    }
    factors
}
