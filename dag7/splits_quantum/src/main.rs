<<<<<<< HEAD
use std::fs;
use std::time::Instant;
=======
use std::{fs, collections::HashMap};
>>>>>>> origin/iterative
fn main() {
    let content = fs::read_to_string("input.txt").expect("path exist");
    let now = Instant::now();
    let Some(sum) = quantum_splits(&content) else {
        // now.elapsed();
        panic!("no sum");
    };
    let elapsed = now.elapsed();
    println!("found sum: {sum}");
    println!("took {:?}", elapsed);
}

fn index_vector(bord: &str) -> Option<usize> {
    let lines: Vec<&str> = bord.lines().collect();
    for i in 0..lines[0].as_bytes().len() {
        if lines[0].as_bytes()[i] == b'S' {
            return Some(i);
        }
    }
    None
}

fn quantum_splits(bord: &str) -> Option<u64> {
<<<<<<< HEAD
    fn path(line: &[&str], index: usize) -> Option<u64> {
        //only have to look at 1 path
        if line.is_empty() {
            return Some(1);
        }
        //length can only be positive
        //length 0 is already covered
        //last line is only dots
        if line.len() == 1 {
            return None;
        }

        //last line is only dots
        if line.len() == 1 {
            return None;
        }
        let mut sum;
        //index.0 = beam index
        if line[0].as_bytes()[index] != b'^' {
            path(&line[2..], index)
        }
        //did find '^'
        else if line.len() == 2{
            Some(2)
        }
        else {
            // println!("slice: {:?}, index {index}", line[0]);
            sum = path(&line[2..], index-1)?;
            // println!("leftsum is: {sum}");
            // println!("slice: {:?}, index {index}", line[0]);
            let temp = path(&line[2..], index+1)?;
            // println!("rightsum is: {temp}");
            sum += temp;
            // println!("total sum is: {sum}");
            Some(sum)

        }
=======
    fn splits_key(beams: &mut HashMap<usize, u64>, index: &usize) ->Option<()> {

        //cant remove, then ignore it
        let Some(amount_beams) = beams.remove(index) else {
            return None;
        };

        println!("found index {} with amount {}", index, amount_beams);

        let beam_left;
        match beams.remove(&(index-1)) {
            Some(amount) => {
                beam_left = amount + amount_beams;
                println!("beam had {} values", amount);
            }
            None => beam_left = amount_beams,
        }

        println!("found index {} with amount {}", index-1, beam_left);

        beams.insert(index-1, beam_left)?;
        let beam_right;
        match beams.remove(&(index+1)) {
            Some(amount) => {
                beam_right = amount + amount_beams;
            }
            None => beam_right = amount_beams,
        }

        println!("found index {}with amount {}", index+1, beam_left);

        beams.insert(index+1, beam_right)?;
        Some(())
>>>>>>> origin/iterative
    }


    let mut beams:HashMap<usize, u64> = HashMap::new();

    let lines: Vec<&str> = bord.lines().collect();
    //find the S
    let index =index_vector(bord)?;
    beams.insert(index, 1);

    for lines in lines[2..].iter().enumerate().step_by(2) {
        println!("line {}", lines.0);
        for bytes_index in 0..lines.1.len() {
            if let b'^' = lines.1.as_bytes()[bytes_index] {
                match splits_key(&mut beams, &bytes_index) {
                    None => continue,
                    Some(_) => (),
                }
            }
        }
    }

    //only even numbers should count 
    let mut sum = 0;
    for (_, v) in beams {
        sum += v;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_input() {
        let content = fs::read_to_string("test_input.txt").expect("path exist");
        assert_eq!(quantum_splits(&content), Some(40));
    }

    #[test]
    fn multiple_splitters() {
        let string =
"..S.......
..........
.^.^......
..........
^.^.^.....
..........";
        let index = index_vector(string);
        assert_eq!(index, Some(2));
        let splits = quantum_splits(string);
        assert_eq!(splits, Some(2));
    }

    #[test]
    fn special_case() {
        let string =
"..S.......
..........
..^.......
..........
.^.^......
..........
^.^.^.....
..........";
        let index = index_vector(string);
        assert_eq!(index, Some(2));
        let splits = quantum_splits(string);
        assert_eq!(splits, Some(8));
    }
}
