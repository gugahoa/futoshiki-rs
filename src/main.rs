use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

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
        m.data.push((i as u32, Box::new(|index: u32| -> i8 { 0 })));
    }

    let path = Path::new("trivial.dat");
    let mut bufreader = match File::open(&path) {
        Err(why) => {
            panic!("Couldn't open file {}: {}",
                   path.display(),
                   why.description())
        }
        Ok(file) => BufReader::new(file),
    };

    for line in bufreader.lines() {
        println!("{}", line.unwrap());
    }

    println!("{}", m.get(3, 3).unwrap().0);
}
