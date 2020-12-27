pub struct PascalsTriangle {
    row_count : u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count : row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.row_count == 0 {
            vec![]
        } else {
            let mut rows : Vec<Vec<u32>> = vec![vec![1]];
            for _ in 0..self.row_count-1 {
                let prev = rows.last().unwrap();
                let mut crt = Vec::with_capacity(prev.len()+1);
                crt.push(1);
                crt.extend_from_slice(&prev);
                for i in 1..prev.len() {
                    crt[i] += crt[i+1]
                }
                rows.push(crt);
            }
            rows
        }
    }
}
