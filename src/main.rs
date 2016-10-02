struct Matrix {
    rows: u32,
    cols: u32,
    data: Vec<(u32, Box<(Fn(u32) -> i8)>)>,
}

impl Matrix {
    fn new(rows: u32, cols: u32) -> Matrix {
        Matrix {
            rows: rows,
            cols: cols,
            data: Vec::new(),
        }
    }

    fn get(&self, x: u32, y: u32) -> Option<&(u32, Box<(Fn(u32) -> i8)>)> {
        if x >= self.rows {
            return None;
        }

        if y >= self.cols {
            return None;
        }

        Some(&self.data[(x + y * self.rows) as usize])
    }
}

fn main() {
    let mut m = Matrix::new(4, 4);
    for i in 0..16 {
        m.data.push((i as u32,
                     Box::new(|index: u32| -> i8 {
            if index == 12 {
                -1
            } else {
                0
            }
        })));
    }

    print!("{}", m.get(3, 3).unwrap().0);
}
