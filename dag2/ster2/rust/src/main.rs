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

//alles moet losgekoppeld worden door komma's
//first id en last id is losgekoppeld met 1 '-'
//leading 0 weg laten
//alle verkeerde id's hebben patronen

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

use itertools;

//beste om deze functies als een aparte module te maken. Geen tijd om dat nu te doen. 

fn range_to_wrong_id(ranges: [u64;2]) -> Option<Vec<u64>> {
    let mut result:Vec<u64> = Vec::new();
    for i in ranges[0]..=ranges[1] {
        //hier moet de find func gebruiken.
        let length = itertools::iterate(i, |&i| i / 10).take_while(|&i| i > 0).count().max(1);
        if length % 2 == 1 {
            continue;
        }
        let length= (length as f32/2.0).round() as u32;
        let left = i/(10_u64.pow(length));
        let right = i%(10_u64.pow(length));
        if left == right {
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
    fn get_sum() {
        let input = str_to_range_int("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        let all_wrong_id = [11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859];
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
            for i in id {
                let wrong_id = iter_wrong_id.next().unwrap_or(&0);
                assert_eq!(*wrong_id, i, "detected id is not wrong");
                println!("the id is: {i}, sum now is: {sum}");
                sum = sum + i;
            }
        }
        assert_eq!(None, iter_wrong_id.next(), "not all values has been read");
        assert_eq!(sum, 1227775554, "sum is {sum}, not 1227775554\n");
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