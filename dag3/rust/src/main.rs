
use std::{fs, str::Chars};

fn main() {
    let content = fs::read_to_string("../input.txt").expect("expect a file");
    let output = find_high_banks(&content);
    let mut sum = 0;
    for num in output {
        match num {
            Some(t) => sum += t,
            None => {
                println!("something went wrong...");
            }
        }
    }
    println!("max joltage is: {sum}");
}

fn combine_val (num: (u8, u8)) ->u8 {
    let (x, y) = num;
    x*10 + y
}

fn find_highest_val(nums: (u8, [u8;2])) -> (u8, u8) {
    //received value (not array)  can't go anywhere except first place
    //second place array (index 1) can go to first place (index 0)
    //first place array (index 0) can go to second place (index1)
    // but will always result in a lower value (21> 02)
    let (input, high) = nums;
    let all_vals =         [combine_val((high[0], high[1])), 
                                     combine_val((high[0], input)), 
                                     combine_val((high[1], input))];
    //is val0 bigger than 1?
    if all_vals[0] > all_vals[1] {
        if all_vals[0] > all_vals[2] {
            //val in index0 is the biggest
            (high[0], high[1])
        }
        //val2 is bigger than 0
        //bit is it bigger than 1?
        else if all_vals[2] > all_vals[1] {
            (high[1], input)
        }
        //val1 is bigger than 2
        else {
            (high[0], input)
        }
    }
    //val0 is not bigger than 1
    else {
        //is val1 bigger than val2?
        if all_vals[1] > all_vals[2] {
            (high[0], input)
        }
        //val 2 is bigger than val1
        else {
            (high[2], input)
        }
    }
}

fn find_high_banks (input: &str) ->Vec<Option<u8>>{
    let banks = str_to_banks(input);
    //be as greedy as possible
    let mut highest_nums = [0_u8,0_u8];
    let mut result = Vec::new();
    for c in banks {
        for character in c {
            let Some(character) = character.to_digit(10) else {
                //not a number!?
                result.push(None);
                break;//don't continue this bank
            };
            
            //find place for the highest value
            //character is 1 digit. no way bigger than u8
            let result = find_highest_val((character as u8, highest_nums));
            highest_nums[0] = result.0;
            highest_nums[1] = result.1;
        }
        result.push(Some(combine_val(highest_nums.into())));
    }
    result
}

fn str_to_banks(input_string: &str) ->Vec<Chars<'_>> {
    
    fn to_str(input: &str) -> Vec<&str> {
        input.lines().collect()
    }

    let banks = to_str(input_string);
    let mut bank = Vec::new();
    for str in banks {
        bank.push(str.chars());
    }
    bank
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_bank() {
        let input = "987654321111111\n
                           818181911112111";

        let output = str_to_banks(input);
        let input = ["987654321111111", "818181911112111"];
        let mut index = 0;
        for mut characters in output {
            let mut expected_str = input[index].chars();
            for _ in 0..input[0].len() {
                assert_eq!(characters.next(), expected_str.next());
            }
            index += 1;
        }
    }

    #[test]
    fn test_banks() {
        let input = "987654321111111\n
                      811111111111119\n
                      234234234234278\n
                      818181911112111";
        let output = [98, 89, 78, 92];
        let expected_sum: u64 = output.iter().sum();
        let result = find_high_banks(input);
        let mut sum: u64 = 0;
        for num in result {
            match num {
                Some(t) => sum += t as u64,
                None => {
                    println!("this isn't supposed to happen!?");
                    panic!();
                }
            }
        }
        assert_eq!(sum, expected_sum, "sums are not equal")
    }

    #[test]
    fn read_input() {
        let content = fs::read_to_string("../test_input.txt").expect("expect a file");
        //krijg de ranges (11-22 en 100-303 bijv.)
        let ranges = content.split(',');
        let mut check_range: Vec<[u64; 2]> = Vec::new();
        //in een vector, sla de begin en eind in een array
        for string_range in ranges {
            let mut iterator = string_range.split('-');
            let array_range = [iterator.next().unwrap(), iterator.next().unwrap()];
            let array_range: [u64;2] = [array_range[0].parse().unwrap(), array_range[1].parse().unwrap()];
            check_range.push(array_range);
        }
        assert_eq!(check_range[0][0], 5542145);
        assert_eq!(check_range[0][1], 5582046);

        assert_eq!(check_range[1][0], 262248430);
        assert_eq!(check_range[1][1], 262271846);

        assert_eq!(check_range[2][0], 211488);
        assert_eq!(check_range[2][1], 230593);
    }
}