use std::{fs, collections::HashSet};
fn main() {
    let content = fs::read_to_string("input.txt").expect("path exist");
    let Some(sum) = quantum_splits(&content) else {
        panic!("no sum");
    };
    println!("found sum: {sum}");
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
    fn path(line: Vec<&str>, index: usize, mut sum: u64) -> Option<u64> {
        let Some(mut sum) = Some(sum) else {
            return None;
        };
        //only have to look at 1 path
        if line.is_empty() {
            return None;
        }
        if line.len() == 0 {
            return Some(sum);
        }
        //index.0 = beam index
        for bytes_index in 0..line[0].len() {
           if let b'^' = line[0].as_bytes()[bytes_index] {
                if bytes_index != index {
                    continue;
                }
                sum += 1;
                let mut sub_path: Vec<&str> = Vec::new();
                for i in 2..line.len() {
                    sub_path.push(line[i]);
                }
                let Some(temp_val) = path(sub_path, bytes_index-1, sum) else {
                    return None;
                }
                sum += temp_val;
                let Some(temp_val) = path(sub_path, bytes_index+1, sum) else {
                    return None;
                };
                sum += temp_val;
            }
        }
        Some(sum)
    }

    let lines: Vec<&str> = bord.lines().collect();
    //find the S
    let index =index_vector(bord)?;
    //only even numbers should count 
    let sum = path(lines, index, 0)?;
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
".....S.......   
.............
....^.^......
.............
...^.^.^.....";
        let index = index_vector(string);
        assert_eq!(index, Some(5));
        let splits = quantum_splits(string);
        assert_eq!(splits, Some(40));
    }
}