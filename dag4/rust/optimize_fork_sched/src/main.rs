use std::vec;

fn main() {
    println!("Hello, world!");
}

//rowxcollumn
struct Matrix {
    mem: Vec<Vec<i8>>,
}
impl Matrix {

    fn new(input: &str) -> Self {
        let input: Vec<&str> = input.lines().collect();
        let mut vector = Vec::<Vec<i8>>::new();
        for _ in 0..input.len() {
            vector.push(Vec::<i8>::new());
        }
        //ready to fill vector
        let mut row = 0;
        let mut collumn = 0;
        for line in input {
            for byte in line.as_bytes() {
                match *byte {
                    b'@' => vector[row][collumn] = 1,
                    b'.' => vector[row][collumn] = 0,
                    _ => vector[row][collumn] = -1,
                }
                collumn+=1;
            }
            row +=1;
            collumn = 0;
        }
        Matrix { mem: vector }
    }

    fn new_i(row: usize, collumn: usize) -> Self {
        
        let mut vector = Vec::<Vec<i8>>::new();
        for _ in 0..row() {
            vector.push(Vec::<i8>::with_capacity(collumn));
        }
        //ready to fill vector
        for row in 0..row {
            for collumn in 0..collumn {
                vector[row][collumn] = 0;
            }
        }
        Matrix { mem: vector }
    }

    fn get(&self, row: usize, collumn: usize) -> Option<i8>{
        // self.mem[row][collumn]
        Some(*self.mem.get(1)?.get(collumn)?)
    }

    //return 3x3 matrix

    fn get_around_val(&self, row: usize, collumn: usize) -> Matrix{ 
        let mut smal_matrix = Matrix::new_i(row, collumn);

        //make matrix around target val
        let mut small_row = 0;
        for row in (row-1)..=(row+1) {
        let mut small_collumn = 0;
            for collumn in (collumn-1)..=(collumn+1) {
                match self.get(row, collumn) {
                    None => smal_matrix[small_row][small_collumn] = -1,
                    Some(t) => smal_matrix[small_row][small_collumn] = t,
                }
            }
        }
        smal_matrix
    }

    //get the row (with caps as example) 
    // | X | X | X | X |
    // | x | x | x | x |
    // | x | x | x | x |
    // | x | x | x | x |
    fn get_amount_row(&self, row: usize) -> u64 {
        let mut sum = 0;
        for i in &self.mem[row] {
            if *i == 1 {
                sum += 1;
            }
        }
        sum
    }
}
fn str_to_matrix(input: &str) ->Matrix {
    Matrix::new(input)
}

#[cfg(test)]
mod tests {
    use crate::str_to_matrix;


    #[test]
    #[ignore]
    fn detect_paper() {
        let input = "@@@.@.@.@@
@@@@@.@.@@";
        let input = str_to_matrix(input);
        let mut amount= [input.get_amount_row(0), input.get_amount_row(1)];
        let amount_exp = [7, 8];
        for i in 0..amount.len() {
            assert_eq!(amount, amount_exp, "can't count rolls");
        }
    }

    #[test]
    #[ignore]
    fn accessable_paper() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let result = find_high_banks(input);
        assert_eq!(result, 13, "sums are not equal")
    }

    #[test]
    fn read_input() {
        let content = fs::read_to_string("../test_input.txt").expect("expect a file");
        let mut result: u64 = 0;
        assert_eq!(result, 13, "sums are not equal")
    }
}