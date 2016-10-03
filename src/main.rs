use std::io;
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
    fn new(dim: u32) -> Matrix {
        Matrix {
            rows: dim,
            cols: dim,
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
    let line_or_panic = || -> String {
        let mut line = String::new();
        if let Err(error) = io::stdin().read_line(&mut line) {
            panic!("Couldn't read line. Error: {}", error);
        }

        String::from(line.trim())
    };

    let n_cases_line = line_or_panic();
    let test_cases = n_cases_line.parse::<u32>().unwrap_or(0);
    println!("{}", n_cases_line);
    for _ in 0..test_cases {
        let u32_values = |line: String| -> Vec<u32> {
            line.trim().split(" ").map(|s| s.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>()
        };

        let first_line = u32_values(line_or_panic());
        let matrix_dim = first_line[0];
        let restrictions = first_line[1];

        let mut matrix = Matrix::new(matrix_dim);
        let mvr_vec = (1..matrix_dim + 1).collect::<Vec<u32>>();
        for i in 0..(matrix_dim * matrix_dim) {
            matrix.mvr.insert(i, mvr_vec.clone());
        }

        for _ in 0..matrix_dim {
            let mut row = u32_values(line_or_panic());
            matrix.data.append(&mut row);
        }

        for _ in 0..restrictions {
            let restriction = u32_values(line_or_panic());

            let (r1, c1) = (restriction[0] - 1, restriction[1] - 1);
            let (r2, c2) = (restriction[2] - 1, restriction[3] - 1);

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

        // consume \n at the end of each case
        line_or_panic();

        matrix.solve(0, 0, 'c');
        for (i, num) in matrix.data.into_iter().enumerate() {
            if (i as u32) % matrix_dim == 0 && i != 0 {
                print!("\n");
            }

            print!("{} ", num);
        }
        println!("\n");
    }
}
