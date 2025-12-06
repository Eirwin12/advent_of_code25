use std::fs;

fn main() {
    let content = fs::read_to_string("../input.txt").expect("expect a file");
    let input = str_to_range_int(&content);
    let mut sum = 0;

    for ranges in input {
        let id;
        match range_to_wrong_id(ranges) {
            Some(vector) => id = vector,
            None => id = Vec::with_capacity(0),
        }
        sum += id.iter().sum::<u64>();
    }
    println!("de gewilde som is: {sum}");
}

fn str_to_range(input: &str) -> Vec<&str>{
    
    //krijg de ranges (11-22 en 100-303 bijv.)
    input.split(',').collect()
}

fn str_to_range_int(input_string: &str) ->Vec<[u64;2]> {
    let ranges = str_to_range(input_string);
    let mut result: Vec<[u64;2]> = Vec::new();
    for i in ranges {
        let mut range = i.split('-');
        let range = [range.next().unwrap(), range.next().unwrap()];
        // {
        //     let value_1 = range[0];
        //     let value_2 = range[1];
        //     println!("range: [{value_1}, {value_2}]");
        // }
        let range: [u64; 2] = [range[0].parse().unwrap(), range[1].parse().unwrap()];
        result.push(range);
    }
    result
}

fn number_to_digit(i: u64) -> Vec<u8> {
    //recursive function
    fn x_inner(n: u64, result: &mut Vec<u8>) {
        if n >= 10 {
            x_inner(n/10, result);
        }
        result.push((n%10) as u8);
    }
    let mut result = Vec::<u8>::new();
    x_inner(i, &mut result);
    result
}

fn range_to_wrong_id(ranges: [u64;2]) -> Option<Vec<u64>> {
    let mut result:Vec<u64> = Vec::new();
    for i in ranges[0]..=ranges[1] {
        //vanaf het waarde na de eerste:
        //initialiseer de patroon met de eerste waarde
        //check of de volgede waarden overeen komen met patroon. 
        //zo wel, dan invalid id
        //zo niet, update de patroon met gevonden reeks en ga door
        //call func om alle digits te splitsen
        let digit: Vec<u8> = number_to_digit(i);
        let mut pattern: Vec<u8> = Vec::new();
        let mut seen_digits: Vec<u8> = Vec::new();

        let mut repeated = false;
        let mut amount_repeat = 0;

        let mut index = 0;
        // println!("now checking {}", i);
        'digit: for i in digit {
            
            //should only happen with first value
            if pattern.is_empty() {
                pattern.push(i);
                continue 'digit;
            }
            seen_digits.push(i);
            

            // println!("index: {index}, pattern val is: {}, seen digit is: {i}", pattern[index]);
            //zo niet, dan moet ik kijken of index klopt met wat gezien is. 
            if pattern[index] == i {
                index+=1;
                if index == pattern.len() {
                    repeated = true;
                    index = 0;
                    amount_repeat += 1;
                    seen_digits.clear();
                    continue 'digit;
                }
                continue 'digit;
            }

            //als de huidige index anders is dan wat gezien is, dan moet pattern geüpdate worden.
            //repeat (als er ooit een repeat was) moet false zijn
            repeated = false;
            let mut new_pattern = Vec::<u8>::new();
            for _ in 0..=amount_repeat {
                new_pattern.append(&mut pattern.clone());
            }
            new_pattern.append(&mut seen_digits.clone());
            amount_repeat = 0;
            pattern = std::mem::take(&mut new_pattern);
            // print!("new pattern is: ");
            // for i in &pattern {
            //     print!("{}", *i);
            // }
            // print!("\n");

            seen_digits.clear();
        }
        if repeated {
            // println!("found value: {i}");
            result.push(i);
        }
    }
    if result.is_empty() {
        None
    }
    else {
        Some(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_ranges() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let output = ["11-22","95-115","998-1012","1188511880-1188511890","222220-222224", "1698522-1698528","446443-446449","38593856-38593862","565653-565659","824824821-824824827","2121212118-2121212124"];
        let result = str_to_range(input);
        for i in 0..11 {
            let output_i = output[i];
            let result_i = result[i];
            assert_eq!(output_i, result_i, "output: {output_i}, result{result_i}");
        }
    }

    #[test]
    fn get_wrong_id() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let output = [[11,22],[95,115],[998,1012],[1188511880,1188511890],[222220,222224], [1698522,1698528],[446443,446449],[38593856,38593862],[565653,565659],[824824821,824824827],[2121212118,2121212124]];
        let result = str_to_range_int(input);
        for i in 0..11 {
            for j in 0..2 {
                let output_i = output[i][j];
                let result_i = result[i][j];
                assert_eq!(output_i, result_i, "output: {output_i}, result{result_i}");                
            }
        }
    }

    #[test]
    fn test_range_10() {
        let input = [[1188511880,1188511890], 
                                    [446443,446449], 
                                    [824824821,824824827]
                                    ];
        let all_wrong_id = [1188511885, 446446, 824824824];
        let mut iter_wrong_id = all_wrong_id.iter();
        let mut sum = 0;
        for ranges in input {
            let id;
            match range_to_wrong_id(ranges) {
                Some(vector) => id = vector,
                None => id = Vec::with_capacity(0),
            }
            //in plaats van iter sum (mogelijk sneller) wordt elke waarde apart opgeteld. 
            //dit, omdat elke waarde toch gecheckt moet worden. 
            {
                let val1 = ranges[0];
                let val2 = ranges[1];
                println!("the range was {val1} to {val2}");
            }
            for i in id {
                let wrong_id = iter_wrong_id.next().unwrap_or(&0);
                assert_eq!(*wrong_id, i, "detected id is not wrong");
                sum = sum + i;
                println!("the id is: {i}, sum now is: {sum}");
            }
        }
        assert_eq!(None, iter_wrong_id.next(), "not all values has been read");
    }

    #[test]
    fn get_sum() {
        let input = str_to_range_int("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        let all_wrong_id = [11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824, 2121212121];
        // let all_wrong_id = [11, 22, 99, 111, 999, 1010, 222222, 38593859, 565656, 2121212121, 824824824, 1188511885, 446446];
        let mut iter_wrong_id = all_wrong_id.iter();
        let mut sum = 0;
        for ranges in input {
            let id;
            match range_to_wrong_id(ranges) {
                Some(vector) => id = vector,
                None => id = Vec::new(),
            }
            //in plaats van iter sum (mogelijk sneller) wordt elke waarde apart opgeteld. 
            //dit, omdat elke waarde toch gecheckt moet worden. 
            {
                let val1 = ranges[0];
                let val2 = ranges[1];
                println!("the range was {val1} to {val2}");
            }
            for i in id {
                let wrong_id = iter_wrong_id.next().unwrap_or(&0);
                println!("{i}");
                assert_eq!(i, *wrong_id, "detected id is not wrong");
                sum = sum + i;
                // println!("the id is: {i}, sum now is: {sum}");
            }
        }
        assert_eq!(None, iter_wrong_id.next(), "not all values has been read");
        assert_eq!(sum, 4174379265, "sum is {sum}, not 4174379265\n");
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