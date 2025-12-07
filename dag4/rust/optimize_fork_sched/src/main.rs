use std::fs;

fn main() {
    let content = fs::read_to_string("../input.txt").expect("expect a file");
    let mut content = Matrix::new(&content);
    let result = content.remove_all_pos_paper(4);
    println!("needed value is: {}", result);
}

//rowxcollumn
struct Matrix {
    mem: Vec<Vec<Place>>,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Place {
    Paper,
    Empty,
    Invalid,
}

impl Place {
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Invalid =>Self::Invalid,
            Self::Paper => Self::Paper,
        }
    }
}
impl Matrix {

    fn new(input: &str) -> Self {
        let input: Vec<&str> = input.lines().collect();
        let mut vector = Vec::<Vec<Place>>::new();
        //ready to fill vector
        let mut row = 0;
        for line in input {
            vector.push(Vec::<Place>::new());
            for byte in line.as_bytes() {
                match *byte {
                    b'@' => vector[row].push(Place::Paper),
                    b'.' => vector[row].push(Place::Empty),
                    _ => vector[row].push(Place::Invalid),
                }
            }
            row +=1;
        }
        Matrix { mem: vector }
    }

    fn new_i(row: usize, collumn: usize) -> Self {
        
        let mut vector = Vec::<Vec<Place>>::new();
        for row in 0..row {
            vector.push(Vec::<Place>::with_capacity(collumn));
            for _ in 0..collumn {
                vector[row].push(Place::Invalid);
            }
        }
        Matrix { mem: vector }
    }

    fn get(&self, row: usize, collumn: usize) -> Option<&Place>{
        // self.mem[row][collumn]
        Some(self.mem.get(row)?.get(collumn)?)
    }

    fn insert(&mut self, row: usize, collumn: usize, value: Place) -> Option<()> {
        //check if place does exist
        if self.mem.get(row) == None {
            return None;
        }
        if self.mem[row].get(collumn) == None {
            return None;
        }
        //ik pas vector zelf niet aan, maar ik pas aan waar het naartoe wijst. 
        self.mem[row][collumn] = value;
        Some(())
    }
    //return 3x3 matrix


    // get 3x3 around row 1, collumn 1
    // | X | X | X | x |
    // | X | x | X | x |
    // | X | X | X | x |
    // | x | x | x | x |

    fn get_around_val(&self, row: usize, collumn: usize) -> Matrix{ 
        //make matrix around target val
        let mut smal_matrix = Matrix::new_i(3, 3);
        
        let mut small_row = 0;
        let row = row as i64;
        let collumn = collumn as i64;
        // println!("row: {row}, low: {low_row}, high: {high_row}");
        // println!("col: {collumn}, low: {low_col}, high: {high_col}");
        for row in row-1..=row+1 {
            if row < 0 || (row as usize) >= self.mem.len(){
                small_row += 1;
                continue;
            }
            let mut small_collumn = 0;
            for collumn in collumn-1..=collumn+1 {
                if collumn <0 || (collumn as usize) >= self.mem[0].len() {
                    small_collumn += 1;
                    continue;
                }
                match self.get(row as usize, collumn as usize) {
                    None => smal_matrix.mem[small_row][small_collumn] = Place::Invalid,
                    Some(t) => smal_matrix.mem[small_row][small_collumn] = t.clone(),
                }
                small_collumn += 1;
            }
        // println!("{:?}", smal_matrix.mem[small_row]);
            small_row += 1;
        }
        //make middle value invalid
        smal_matrix.mem[1][1] = Place::Empty;
        smal_matrix
    }

    //IT SHOULD NOT SUM IF CELL ISN'T PAPER
    fn sum_around_val(&self, row: usize, collumn: usize) ->Option<u8>{
        if self.mem[row][collumn] != Place::Paper {
            return None;
        }
        let mut sum = 0_u8;
        let input = self.get_around_val(row, collumn);
        //sum alles om cel heen
        //is altijd 3x3
        // println!("matrix:\n{:?}\n{:?}\n{:?}\n", input.mem[0], input.mem[1], input.mem[2]);
        for row in 0..3 {
            for collumn in 0..3 {
                if (row == 1) && (collumn == 1) {
                    continue;
                }
                let Some(val) = input.get(row, collumn) else {
                    //place doesn't exist??
                    continue;
                };
                if val == &Place::Paper {
                    sum += 1;
                }
            }
        }
        // println!("sum: {sum}");
        Some(sum)
    }

    fn check_sum_all_3x3(&self, check: u8) ->u64 {
        let mut result = 0_u64;
        for row in 0..self.mem.len() {
            //all vectors are the same length
            for collumn in 0..self.mem[0].len() {
                let Some(sum) = self.sum_around_val(row, collumn)  else {
                    continue;
                };
                // println!("{sum} bigger? {}", sum<check);
                if sum < check {
                    result+=1;
                }
            }
        }
        result
    }
    
    fn remove_paper(&mut self, check: u8) -> (u64, bool){
        let mut sum_removed = 0_u64;
        let mut removed = false;
        for row in 0..self.mem.len() {
            //all vectors are the same length
            for collumn in 0..self.mem[0].len() {
                let Some(sum) = self.sum_around_val(row, collumn)  else {
                    continue;
                };
                // println!("{sum} bigger? {}", sum<check);
                if sum < check {
                    sum_removed +=1;
                    removed = true;
                    if None == self.insert(row, collumn, Place::Empty) {
                        println!("something went wrong...");
                    }
                }
            }
        }
        (sum_removed, removed)
    }

    fn remove_all_pos_paper(&mut self, check: u8) -> u64{
        let mut sum_removed = 0_u64;
        let mut removed = true;
        while removed {
            let removed_val;
            (removed_val, removed) = self.remove_paper(check);
            sum_removed += removed_val;
        }
        sum_removed
    }
}

#[cfg(test)]
mod tests {

    use crate::Place;
    use super::*;

    #[test]
    fn detect_paper() {
        //check paper
        let input = "@@@
@@@
@@@";
        let input = Matrix::new(input);
        for row in input.mem {
            for collumn in row {
                assert_eq!(collumn, Place::Paper, "value is wrong");
            }
        }

        //check invalids
        let input = "aaa
aaa
aaa";
        let input = Matrix::new(input);
        for row in input.mem {
            for collumn in row {
                assert_eq!(collumn, Place::Invalid, "value is wrong");
            }
        }
        
        //check empty
        let input = "...
...
...";
        let input = Matrix::new(input);
        for row in input.mem {
            for collumn in row {
                assert_eq!(collumn, Place::Empty, "value is wrong");
            }
        }
        
        //check combination
        let input = ".a@
.a@
a.a";
        let input = Matrix::new(input);
        let expected_output = [Place::Empty, Place::Invalid, Place::Paper, 
                                           Place::Empty, Place::Invalid, Place::Paper,
                                           Place::Invalid, Place::Empty, Place::Invalid];
        let mut index = 0;
        for row in input.mem {
            for collumn in row {
                assert_eq!(collumn, expected_output[index], "value is wrong");
                index+=1;
            }
        }
    }

    #[test]
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
//row 2 and collumn 1
//111
//111
//101
        let input = Matrix::new(input);
        let result = input.sum_around_val(2, 1);
        assert_eq!(result, Some(7), "sums are not equal");
//row 8 and collumn 4
        let result = input.sum_around_val(8, 4);
        assert_eq!(result, Some(6), "sums are not equal");
//row 9 and collumn 0
//-01
//-10
//---
        let result = input.sum_around_val(9, 0);
        assert_eq!(result, Some(1), "sums are not equal");
    }

    #[test]
    fn sum_all() {
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
        let input = Matrix::new(input);
        let result = input.check_sum_all_3x3(4);
        assert_eq!(result, 13, "sums are not equal");
    }

    #[test]
    fn read_input() {
        let content = fs::read_to_string("../test_input.txt").expect("expect a file");
        let content = Matrix::new(&content);
        let result: u64 = content.check_sum_all_3x3(4);
        assert_eq!(result, 13, "sums are not equal");
    }

    #[test]
    fn rm_paper() {

    }
    #[test]
    fn rm_all_paper() {
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
        let mut input = Matrix::new(input);
        let result = input.remove_all_pos_paper(4);
        assert_eq!(result, 43, "sums are not equal");
    }
    #[test]
    fn rm_read_input() {
        let content = fs::read_to_string("../test_input.txt").expect("expect a file");
        let mut content = Matrix::new(&content);
        let result: u64 = content.remove_all_pos_paper(4);
        assert_eq!(result, 43, "sums are not equal");
    }
}