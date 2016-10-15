use std::io;

trait Futoshiki {
    fn blocking_indexes(&self, r: u32, c: u32) -> Vec<usize>;
    fn next_index(&self, flag: char) -> Option<(u32, u32)>;
    fn update_mvr(&mut self, index_greater: usize, index_lesser: usize);

    fn forward_check(&self, value: u32, blocking_indexes_vec: &Vec<usize>, flag: char) -> bool;
    fn can_put_num(&self, r: u32, c: u32, num: u32, blocking_indexes_vec: &Vec<usize>) -> bool;
    fn solve(&mut self, r: u32, c: u32, flag: char) -> bool;
}

struct Matrix {
    rows: u32,
    cols: u32,
    data: Vec<u32>,
    mvr: Vec<Vec<u32>>,
    cell_restriction: Vec<(u8, Box<(Fn(usize) -> i8)>)>,
    attributions: u64,
}

impl Matrix {
    fn new(dim: u32) -> Matrix {
        Matrix {
            rows: dim,
            cols: dim,
            data: Vec::new(),
            mvr: Vec::new(),
            cell_restriction: Vec::new(),
            attributions: 0,
        }
    }

    fn set(&mut self, r: u32, c: u32, value: u32) {
        self.data[(c + r * self.rows) as usize] = value;
        self.attributions += 1;
    }
}

impl Futoshiki for Matrix {
    fn blocking_indexes(&self, row: u32, col: u32) -> Vec<usize> {
        let mut indexes: Vec<usize> = Vec::new();
        for r in 0..self.rows {
            if r == row {
                continue;
            }

            indexes.push((col + r * self.cols) as usize);
        }

        for c in 0..self.cols {
            if c == col {
                continue;
            }

            indexes.push((c + row * self.cols) as usize);
        }

        indexes
    }

    fn next_index(&self, flag: char) -> Option<(u32, u32)> {
        if flag == 'c' {
            match self.mvr
                .iter()
                .enumerate()
                .filter(|&(index, _)| self.data[index] == 0)
                .min_by_key(|&(_, v)| v.len()) {
                None => return None,
                Some((index, _)) => {
                    let index = index as u32;
                    return Some((index / self.rows, index % self.cols));
                }
            }
        }

        match self.data.iter().position(|&x| x == 0) {
            None => None,
            Some(index) => {
                let index = index as u32;
                Some((index / self.rows, index % self.cols))
            }
        }
    }

    fn forward_check(&self, value: u32, blocking_indexes_vec: &Vec<usize>, flag: char) -> bool {
        if flag == 'a' {
            return true;
        }

        for index in blocking_indexes_vec {
            if self.data[*index] == 0 {
                continue;
            }

            let index_mvr = &self.mvr[*index];
            if index_mvr.contains(&value) && index_mvr.len() == 1 {
                return false;
            }
        }

        return true;
    }

    fn update_mvr(&mut self, index_greater: usize, index_lesser: usize) {
        let ig_max = *self.mvr[index_greater].iter().max().unwrap();
        self.mvr[index_lesser].retain(|&x| x < ig_max);
    }

    fn can_put_num(&self, r: u32, c: u32, num: u32, blocking_indexes_vec: &Vec<usize>) -> bool {
        for index in blocking_indexes_vec {
            let index = *index;
            if self.data[index] == 0 {
                continue;
            }

            if self.data[index] == num {
                return false;
            }

            let (valid, ref fn_restrict) = self.cell_restriction[index];
            if valid == 1 {
                let check = fn_restrict((c + r * self.cols) as usize);
                if check == 1 && self.data[index] > num {
                    return false;
                } else if check == -1 && self.data[index] < num {
                    return false;
                }
            }
        }

        return true;
    }

    fn solve(&mut self, r: u32, c: u32, flag: char) -> bool {
        let possible_nums;
        if flag != 'c' {
            possible_nums = (1..self.cols + 1).collect();
        } else {
            let index = c + r * self.cols;
            possible_nums = self.mvr[index as usize].clone();
        }

        let mut removed_stack = Vec::new();
        for possible_num in possible_nums {
            let blocking_indexes_vec = self.blocking_indexes(r, c);
            if self.can_put_num(r, c, possible_num, &blocking_indexes_vec) &&
               self.forward_check(possible_num, &blocking_indexes_vec, flag) {
                if self.attributions > 1000000 {
                    return false;
                }

                self.set(r, c, possible_num);
                if flag == 'c' {
                    for index in &blocking_indexes_vec {
                        if let Some(possible_num_index) = self.mvr[*index]
                            .iter()
                            .position(|&x| x == possible_num) {
                            removed_stack.push(*index);
                            self.mvr[*index].swap_remove(possible_num_index);
                        };
                    }
                }

                match self.next_index(flag) {
                    None => return true,
                    Some((next_r, next_c)) => {
                        if self.solve(next_r, next_c, flag) {
                            return true;
                        }
                    }
                }

                if flag == 'c' {
                    for index in &removed_stack {
                        self.mvr[*index].push(possible_num);
                    }

                    removed_stack.clear();
                }
            }
        }

        self.set(r, c, 0);
        return false;
    }
}

fn restriction_func(maybe_old_f: (u8, Box<(Fn(usize) -> i8)>),
                    cell: usize,
                    value: i8)
                    -> Box<(Fn(usize) -> i8)> {

    let (valid, old_f) = maybe_old_f;
    Box::new(move |index: usize| -> i8 {
        if valid == 1 {
            let ret = old_f(index);
            if ret != 0 {
                return ret;
            }
        }

        if index == cell {
            value
        } else {
            0
        }
    })
}

fn main() {
    let line_or_panic = || -> String {
        let mut line = String::new();
        if let Err(error) = io::stdin().read_line(&mut line) {
            panic!("Couldn't read line. Error: {}", error);
        }

        String::from(line.trim())
    };

    let u32_values = |line: String| -> Vec<u32> {
        line.trim().split(" ").map(|s| s.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>()
    };

    let n_cases_line = line_or_panic();
    let test_cases = n_cases_line.parse::<u32>().unwrap_or(0);
    println!("{}", n_cases_line);
    for _ in 0..test_cases {
        let first_line = u32_values(line_or_panic());
        let matrix_dim = first_line[0];
        let restrictions = first_line[1];

        let mut matrix = Matrix::new(matrix_dim);
        let mvr_vec = (1..matrix_dim + 1).collect::<Vec<u32>>();

        for i in 0..(matrix_dim * matrix_dim) {
            if i < matrix_dim {
                let mut row = u32_values(line_or_panic());
                matrix.data.append(&mut row);
            }

            matrix.mvr.push(mvr_vec.clone());
            matrix.cell_restriction.push((0, Box::new(|_: usize| -> i8 { 0 })));
        }

        for _ in 0..restrictions {
            let restriction = u32_values(line_or_panic());

            let (r1, c1) = (restriction[0] - 1, restriction[1] - 1);
            let (r2, c2) = (restriction[2] - 1, restriction[3] - 1);

            let index1 = (c1 + r1 * matrix_dim) as usize;
            let index2 = (c2 + r2 * matrix_dim) as usize;

            matrix.update_mvr(index2, index1);

            let index_max = std::cmp::max(index1, index2);
            let index_min = std::cmp::min(index1, index2);

            let old_f1 = matrix.cell_restriction.remove(index_max);
            let old_f2 = matrix.cell_restriction.remove(index_min);

            let index_min_fn = restriction_func(old_f1,
                                                index_max,
                                                if index_max == index2 {
                                                    1
                                                } else {
                                                    -1
                                                });
            let index_max_fn = restriction_func(old_f2,
                                                index_min,
                                                if index_min == index2 {
                                                    1
                                                } else {
                                                    -1
                                                });

            matrix.cell_restriction.insert(index_min, (1, index_min_fn));
            matrix.cell_restriction.insert(index_max, (1, index_max_fn));
        }

        // consume \n at the end of each case
        line_or_panic();

        if !matrix.solve(0, 0, 'c') {
            println!("Numero de atribuicoes excede o limite");
            continue;
        }
        for (i, num) in matrix.data.into_iter().enumerate() {
            if (i as u32) % matrix_dim == 0 && i != 0 {
                print!("\n");
            }

            print!("{} ", num);
        }
        println!("\n");
    }
}
