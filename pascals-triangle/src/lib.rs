pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut pt = PascalsTriangle(vec![]);
        for pos in 0..=row_count {
            let mut row = vec![];
            for row_pos in 0..=pos {
                let mut value = 1;
                if let Some(x) = pt.0.get((pos - 1) as usize) {
                    match (x.get((row_pos - 1) as usize), x.get((row_pos + 1) as usize)) {
                        (Some(p), Some(a)) => value += p + a,
                        (None, Some(a)) => value += a,
                        (Some(p), None) => value += p,
                        _ => {}
                    }
                }
                row.push(value);
            }
            pt.0.push(row);
        }
        pt
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
