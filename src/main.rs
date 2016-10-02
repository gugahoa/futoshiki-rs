use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

trait Futoshiki {
    fn forward_check(&self, x: u32, y: u32, value: u32, flag: char) -> bool;

    fn can_put_num(&self, x: u32, y: u32, num: u32) -> bool;
    fn solve(&self, x: u32, y: u32, flag: char) -> bool;
}

struct Matrix {
    rows: u32,
    cols: u32,
    data: Vec<u32>,
    mvr: Vec<Vec<u32>>,
    cell_restriction: HashMap<u32, Box<(Fn(u32) -> i8)>>,
}

impl Matrix {
    fn new(rows: u32, cols: u32) -> Matrix {
        Matrix {
            rows: rows,
            cols: cols,
            data: Vec::new(),
            mvr: Vec::new(),
            cell_restriction: HashMap::new(),
        }
    }

    fn get(&self, x: u32, y: u32) -> Option<u32> {
        if x >= self.rows {
            return None;
        }

        if y >= self.cols {
            return None;
        }

        Some(self.data[(x + y * self.rows) as usize])
    }
}

impl Futoshiki for Matrix {
    fn forward_check(&self, x: u32, y: u32, value: u32, flag: char) -> bool {
        if flag == 'a' {
            return true;
        }

        for row in 0..self.rows {
            let index_mvr = &self.mvr[(row + y * self.rows) as usize];
            if index_mvr.contains(&value) && index_mvr.len() == 1 {
                return false;
            }
        }

        for col in 0..self.cols {
            let index_mvr = &self.mvr[(x + col * self.rows) as usize];
            if index_mvr.contains(&value) && index_mvr.len() == 1 {
                return false;
            }
        }

        return true;
    }

    fn can_put_num(&self, x: u32, y: u32, num: u32) -> bool {
        for row in 0..self.rows {
            if self.get(row, y).unwrap() == num {
                return false;
            }
        }

        for col in 0..self.cols {
            if self.get(x, col).unwrap() == num {
                return false;
            }
        }

        return true;
    }

    fn solve(&self, x: u32, y: u32, flag: char) -> bool {
        true
    }
}

fn main() {
    let path = Path::new("trivial.dat");
    let mut bufreader = match File::open(&path) {
        Err(why) => {
            panic!("Couldn't open file {}: {}",
                   path.display(),
                   why.description())
        }
        Ok(file) => BufReader::new(file),
    };
    let mut first_line = String::new();
    match bufreader.read_line(&mut first_line) {
        Err(why) => panic!("Couldn't read line: {}", why.description()),
        _ => {}
    }

    let u32_values: Vec<u32> = first_line.trim()
        .split(" ")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", u32_values);

    let fn_num = u32_values[1];
    let matrix_dim = u32_values[0];
    let mut matrix = Matrix::new(matrix_dim, matrix_dim);

    let mut mvr_vec: Vec<u32> = Vec::new();
    for i in 0..matrix_dim {
        mvr_vec.push(i + 1);
    }

    println!("{:?}", mvr_vec);

    let mut count = 0;
    for line in bufreader.lines() {
        let mut u32_values: Vec<u32> = line.unwrap()
            .split(" ")
            .map(|s| {
                s.parse::<u32>()
                    .unwrap()
            })
            .collect();

        if count < matrix_dim {
            matrix.data.append(&mut u32_values);
            matrix.mvr.push(mvr_vec.clone());
            count += 1;
            continue;
        }

        let (x1, y1) = (u32_values[0], u32_values[1]);
        let (x2, y2) = (u32_values[2], u32_values[3]);

        let index1 = x1 + y1 * matrix_dim;
        let index2 = x2 + y2 * matrix_dim;

        let maybe_old_f1 = matrix.cell_restriction.remove(&index1);
        let maybe_old_f2 = matrix.cell_restriction.remove(&index2);
        let x1y1_fn = move |index| -> i8 {
            if let Some(ref old_f1) = maybe_old_f1 {
                let ret = old_f1(index);
                if ret != 0 {
                    return ret;
                }
            }

            if index == index2 {
                1
            } else {
                0
            }
        };
        let x2y2_fn = move |index| -> i8 {
            if let Some(ref old_f2) = maybe_old_f2 {
                let ret = old_f2(index);
                if ret != 0 {
                    return ret;
                }
            }

            if index == index1 {
                -1
            } else {
                0
            }
        };

        matrix.cell_restriction.insert(index1, Box::new(x1y1_fn));
        matrix.cell_restriction.insert(index2, Box::new(x2y2_fn));
    }

    let cell_6 = &matrix.cell_restriction[&6];
    println!("{}, {}", cell_6(5), cell_6(10));
}
