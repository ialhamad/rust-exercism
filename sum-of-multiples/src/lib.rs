pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = Vec::new();
    for n in factors {
        if n > &0 {
            for i in (*n..limit).step_by(*n as usize) {
                multiples.push(i)
            }
        }
    }
    multiples.sort();
    multiples.dedup();
    multiples.into_iter().sum()
}