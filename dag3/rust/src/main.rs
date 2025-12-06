
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
            (high[1], input)
        }
    }
}

fn find_high_banks (input: &str) ->Vec<Option<u8>>{
    let banks = str_to_banks(input);
    //be as greedy as possible
    let mut result = Vec::new();
    for c in banks {
        let mut highest_nums = [0_u8,0_u8];
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
        let mut result: Vec<&str> = input.lines().collect();
        for i in 0..result.len() {
            result[i] = result[i].trim();
        }
        result
    }

    let banks = to_str(input_string);
    println!("strings are: \n{}\n{}", banks[0], banks[1]);
    if banks.get(1) == None {
        println!("second string isn't being detected");
    }
    else {
        println!("found string: {}", banks[1]);
    }
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
        let input = "987654321111111
                           818181911112111";

        let output = str_to_banks(input);
        let input = ["987654321111111", "818181911112111"];
        let mut index = 0;
        for mut characters in output {
            let mut expected_str = input[index].chars();
            for _ in 0..input[0].len() {
                let chars = [characters.next(), expected_str.next()];
                
                println!("found chars are:\n{:?}\n{:?}", chars[0], chars[1]);
                assert_eq!(chars[0], chars[1]);
            }
            index += 1;
        }
    }

    #[test]
    fn test_banks() {
        let input = "987654321111111
                      811111111111119
                      234234234234278
                      818181911112111";
        let output = [98, 89, 78, 92];
        let mut index = 0;
        let expected_sum: u64 = output.iter().sum();
        let result = find_high_banks(input);
        let mut sum: u64 = 0;
        for num in result {
            match num {
                Some(t) => {
                    sum += t as u64;
                    assert_eq!(t, output[index] as u8, "wrong sum");
                    index+=1;
                }
                None => {
                    println!("this isn't supposed to happen!?");
                    panic!();
                }
            }
        }
        assert_eq!(sum, expected_sum, "sums are not equal")
    }

    // #[test]
    // fn read_input() {
    //     let content = fs::read_to_string("../test_input.txt").expect("expect a file");
    // }
}