use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

trait Futoshiki {
    fn forward_check(&self, r: u32, c: u32, value: u32, flag: char) -> bool;
    fn next_index(&self, flag: char) -> Option<(u32, u32)>;
    fn blocking_indexes(&self, r: u32, c: u32) -> Vec<u32>;

    fn can_put_num(&self, r: u32, c: u32, num: u32) -> bool;
    fn solve(&mut self, r: u32, c: u32, flag: char) -> bool;
}

struct Matrix {
    rows: u32,
    cols: u32,
    data: Vec<u32>,
    mvr: HashMap<u32, Vec<u32>>,
    cell_restriction: HashMap<u32, Box<(Fn(u32) -> i8)>>,
}

impl Matrix {
    fn new(rows: u32, cols: u32) -> Matrix {
        Matrix {
            rows: rows,
            cols: cols,
            data: Vec::new(),
            mvr: HashMap::new(),
            cell_restriction: HashMap::new(),
        }
    }

    fn set(&mut self, r: u32, c: u32, value: u32) {
        if r >= self.rows || c >= self.cols {
            return;
        }

        self.data[(c + r * self.rows) as usize] = value;
    }
}

impl Futoshiki for Matrix {
    fn blocking_indexes(&self, row: u32, col: u32) -> Vec<u32> {
        let mut indexes: Vec<u32> = Vec::new();
        for r in 0..self.rows {
            if r == row {
                continue;
            }

            indexes.push(col + r * self.cols);
        }

        for c in 0..self.cols {
            if c == col {
                continue;
            }

            indexes.push(c + row * self.cols);
        }

        indexes
    }

    fn forward_check(&self, r: u32, c: u32, value: u32, flag: char) -> bool {
        if flag == 'a' {
            return true;
        }

        for index in self.blocking_indexes(r, c)
            .into_iter()
            .filter(|&i| self.data[i as usize] == 0) {
            let index_mvr = &self.mvr[&index];
            if index_mvr.contains(&value) && index_mvr.len() == 1 {
                return false;
            }
        }

        return true;
    }

    fn next_index(&self, flag: char) -> Option<(u32, u32)> {
        if flag == 's' {
            return Some((0, 0));
        }

        if flag == 'a' {
            match self.data.iter().position(|&x| x == 0) {
                None => None,
                Some(index) => {
                    let index = index as u32;
                    Some((index / self.rows, index % self.cols))
                }
            }
        } else {
            match self.mvr
                .iter()
                .filter(|&(i, _)| self.data[*i as usize] == 0)
                .min_by_key(|&(_, value)| value) {
                None => None,
                Some((&index, _)) => Some((index / self.rows, index % self.cols)),
            }
        }
    }

    fn can_put_num(&self, r: u32, c: u32, num: u32) -> bool {
        for index in self.blocking_indexes(r, c)
            .into_iter()
            .filter(|&i| self.data[i as usize] != 0) {
            if self.data[index as usize] == num {
                return false;
            }

            if let Some(fn_restrict) = self.cell_restriction.get(&index) {
                let check = fn_restrict(c + r * self.cols);
                if check == 1 && self.data[index as usize] > num {
                    return false;
                } else if check == -1 && self.data[index as usize] < num {
                    return false;
                }
            };
        }

        return true;
    }

    fn solve(&mut self, r: u32, c: u32, flag: char) -> bool {
        let mut possible_nums = Vec::new();
        if flag == 'a' {
            for i in 1..(self.cols + 1) {
                possible_nums.push(i);
            }
        } else {
            let index = c + r * self.cols;
            possible_nums.append(&mut self.mvr[&index].clone());
        }

        for possible_num in possible_nums {
            if self.can_put_num(r, c, possible_num) &&
               self.forward_check(r, c, possible_num, flag) {
                self.set(r, c, possible_num);

                match self.next_index(flag) {
                    None => return true,
                    Some((next_r, next_c)) => {
                        if self.solve(next_r, next_c, flag) {
                            return true;
                        }
                    }
                }
            }
        }

        self.set(r, c, 0);
        return false;
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

    let matrix_dim = u32_values[0];
    let mut matrix = Matrix::new(matrix_dim, matrix_dim);

    let mut mvr_vec: Vec<u32> = Vec::new();
    for i in 0..matrix_dim {
        mvr_vec.push(i + 1);
    }

    for i in 0..(matrix_dim * matrix_dim) {
        matrix.mvr.insert(i, mvr_vec.clone());
    }

    let mut count = 0;
    for line in bufreader.lines() {
        let mut u32_values: Vec<u32> = line.unwrap()
            .split(" ")
            .map(|s| {
                let v = s.parse::<u32>()
                    .unwrap();
                if count >= matrix_dim {
                    v - 1
                } else {
                    v
                }
            })
            .collect();

        if count < matrix_dim {
            matrix.data.append(&mut u32_values);
            count += 1;
            continue;
        }

        let (r1, c1) = (u32_values[0], u32_values[1]);
        let (r2, c2) = (u32_values[2], u32_values[3]);

        let index1 = c1 + r1 * matrix_dim;
        let index2 = c2 + r2 * matrix_dim;

        let maybe_old_f1 = matrix.cell_restriction.remove(&index1);
        let maybe_old_f2 = matrix.cell_restriction.remove(&index2);
        let r1c1_fn = move |index| -> i8 {
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
        let r2c2_fn = move |index| -> i8 {
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

        matrix.cell_restriction.insert(index1, Box::new(r1c1_fn));
        matrix.cell_restriction.insert(index2, Box::new(r2c2_fn));
    }

    if let Some((start_r, start_c)) = matrix.next_index('s') {
        matrix.solve(start_r, start_c, 'c');
        for (i, num) in matrix.data.iter().enumerate() {
            if (i as u32) % matrix_dim == 0 && i != 0 {
                print!("\n");
            }
            print!("{} ", num);
        }
    };
    println!("");
}
