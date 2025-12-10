use std::fs;
fn main() {
    let content = fs::read_to_string("input.txt").expect("path exist");
    let sum = beams::count_splits(&content);
    println!("found sum: {sum}");
}

mod beams {

    pub fn count_splits(bord: &str) -> u64 {
        let lines: Vec<&str> = bord.lines().collect();
        //find the S
        let mut beams: Vec<usize> = Vec::new();
        for i in 0..lines[0].as_bytes().len() {
            if lines[0].as_bytes()[i] == b'S' {
                beams.push(i);
                break;
            }
        }

        let mut sum = 0;
        //only even numbers should count
        for i in 2..lines.len() {
            if i%2 != 0 {
                continue;
            }
            for bytes in 0..lines[i].len() {
                if let b'^' = lines[i].as_bytes()[bytes] {
                    if !beams.contains(&bytes) {
                        continue;
                    }
                    beams.remove(beams.iter().position(|x| *x == bytes).expect("needle not found"));
                    beams.push(bytes+1);
                    beams.push(bytes-1);
                    sum += 1;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_input() {
        let content = fs::read_to_string("test_input.txt").expect("path exist");
        assert_eq!(beams::count_splits(&content), 21);
    }
}