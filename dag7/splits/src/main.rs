use std::{fs, collections::HashSet};
fn main() {
    let content = fs::read_to_string("input.txt").expect("path exist");
    let Some(sum) = count_splits(&content) else {
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

fn count_splits(bord: &str) -> Option<u64> {
    let lines: Vec<&str> = bord.lines().collect();
    //find the S
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(index_vector(bord)?);
    let mut sum = 0;
    //only even numbers should count
    for line_index in (2..lines.len()).step_by(2) {
        //need the index
        for bytes_index in 0..lines[line_index].len() {
            if let b'^' = lines[line_index].as_bytes()[bytes_index] {
                //cant remove, then ignore it
                if !beams.remove(&bytes_index) {
                    continue;
                }
                beams.insert(&bytes_index-1);
                beams.insert(&bytes_index+1);
                sum += 1;
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_input() {
        let content = fs::read_to_string("test_input.txt").expect("path exist");
        assert_eq!(count_splits(&content), Some(21));
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
        let splits = count_splits(string);
        assert_eq!(splits, Some(1));

        let string =
".....S.......   
.............
.....^.......
.............
....^.^......
.............
...^.^.^.....";
        let splits = count_splits(string);
        assert_eq!(splits, Some(6));

        let string = fs::read_to_string("second_test.txt").unwrap();
        let splits = count_splits(&string);
        assert_eq!(splits, Some(25));
    }
}