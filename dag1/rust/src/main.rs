enum Rotate 
{
    Left,
    Right,
    None
}

struct Movement
{
    direction: Rotate,
    steps: i16,
}

use std::fs;

fn main() {
    let path = "../input.txt";
    let contents = fs::read_to_string(path).expect("file does not exist");
    let mut place: i16 = 50;
    let mut amount_0:u32 = 0;

    for line in contents.lines() {
        let instruction = str_to_movement(line);
        place = execute_instruction(instruction, place);
        if place == 0 {
            amount_0 += 1;
        }
    }
    println!("answer is {amount_0}");
}

fn execute_instruction(move_p: Movement, place: i16) ->i16
{
    let input: i16 = place;
    match move_p.direction {
        Rotate::Left => (input - move_p.steps).rem_euclid(100),
        Rotate::Right => (input + move_p.steps).rem_euclid(100),
        _ => input,
    }
}

fn str_to_movement( slice: &str) -> Movement{
    let mut char = slice.chars();
    let richting;
    match char.next() {
        Some('R') => richting = Rotate::Right,
        Some('L') => richting = Rotate::Left,
        _ => richting = Rotate::None,
    }
    let value = char.as_str().parse::<i16>().or::<i16>(Ok(0)).unwrap();
    
    
    Movement { direction: richting, steps: value }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_move() {
        {
            //normale beweging
            let input: i16 = 11;
            let instruction = Movement{ direction: Rotate::Right, steps: 8};
            let output = execute_instruction(instruction, input);
            assert_eq!(output, 19, "output ({output}) is not 19");
            
            let input = output;
            let instruction = Movement { direction: Rotate::Left, steps: 19 };
            let output = execute_instruction(instruction, input);
            assert_eq!(output, 0, "output ({output}) is not 0");
        }
        {
            //cirkel bewweging
            let input: i16 = 5;
            let instruction = Movement{ direction: Rotate::Left, steps: 10};
            let output = execute_instruction(instruction, input);
            assert_eq!(output, 95, "output ({output}) is not 95");
            
            let input = output;
            let instruction = Movement { direction: Rotate::Right, steps: 5 };
            let output = execute_instruction(instruction, input);
            assert_eq!(output, 0, "output ({output}) is not 0");
        }
    }
    #[test]
    fn count_zero() {
        let list_of_instructions = [
            Movement {direction: Rotate::Left, steps: 68},
            Movement {direction: Rotate::Left, steps: 30},
            Movement {direction: Rotate::Right, steps: 48},
            Movement {direction: Rotate::Left, steps: 5},
            Movement {direction: Rotate::Right, steps: 60},
            Movement {direction: Rotate::Left, steps: 55},
            Movement {direction: Rotate::Left, steps: 1},
            Movement {direction: Rotate::Left, steps: 99},
            Movement {direction: Rotate::Right, steps: 14},
            Movement {direction: Rotate::Left, steps: 82},
        ];
        let mut input: i16 = 50;
        let mut amount_0: u32 = 0;
        for i in list_of_instructions {
            input = execute_instruction(i, input);
            if input == 0 {
                amount_0 += 1;
            }
        }
        assert_eq!(amount_0, 3);
    }
    #[test]
    fn read_input() {
        let content = fs::read_to_string("../test_input.txt").expect("expect a file");
        let mut lines = content.lines();
        assert_eq!(lines.next(), Some("R41"));
        assert_eq!(lines.next(), Some("L10"));
        {
            let content = fs::read_to_string("../test_input.txt").expect("expect a file");
            let mut lines = content.lines();
            let instruction = str_to_movement(lines.next().unwrap());
            
            match instruction.direction {
                Rotate::Right => println!("expected output"),
                _ => println!("something is wrong"),
            }


            assert_eq!(instruction.steps, 41, "instruction is not expected output");
            let instruction = str_to_movement(lines.next().unwrap());

            match instruction.direction {
                Rotate::Left => println!("expected output"),
                _ => println!("something is wrong"),
            }

            assert_eq!(instruction.steps, 10, "instruction is not expected output");
        }
    }
}