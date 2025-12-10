use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("expect a file");
    let (ranges, values) = id_range::split_range_value(&content);
    let ranges = id_range::vec_range(ranges);
    let mut fresh = 0;
    for i in values {
        match id_range::in_range(&ranges, i) {
            Some(_) => fresh += 1,
            _ => (),
        }
    }
    let sum = id_range::all_valid(&ranges);
    println!("amount of fresh ingredient: {fresh}");

    println!("sum fresh ingredient: {sum}");
}

mod id_range {

    fn merger(range_l: &(u64, u64), range_r: &(u64, u64)) -> Option<(u64, u64)>{
        //range_l begin -- range_r begin -- range_r end -- range_l end
        if range_l.0 <range_r.0 && range_l.1> range_r.1 {
            return Some((range_l.0, range_l.1));
        }
        //range_r begin -- range_l begin -- range_l end -- range_r end
        if range_r.0 <range_l.0 && range_r.1> range_l.1 {
            return Some((range_r.0, range_r.1));
        }

        //range_l begin -- range_r begin -- range_l end -- range_r end
        if range_l.0 <range_r.0 && range_r.0<range_l.1 && range_l.1<=range_r.1 {
            return Some((range_l.0, range_r.1));
        }
        //range_r begin -- range_l begin -- range_r end -- range_l end
        if range_r.0 <range_l.0 && range_l.0<range_r.1 && range_r.1<=range_l.1 {
            return Some((range_r.0, range_l.1));
        }
        //same, return 1
        if range_l == range_r {
            return Some((range_l.0, range_l.1));
        }
        None
    }

    fn optimize_range(ranges: &mut Vec<(u64, u64)>) {
        let mut merged: bool;
        loop {
            merged = false;
            let mut result = Vec::new();
            //fill result with ranges values
            // println!("ranges is: {:?}", ranges);
            for i in 0..ranges.len() {
                for j in i+1..ranges.len() {
                    let Some(merged_range) = merger(&ranges[i], &ranges[j]) else {
                        result.push(ranges[i].clone());
                        result.push(ranges[j].clone());
                        continue;
                    };
                    if i == 0 {
                        // println!("found range: {:?}", merged_range);
                    }
                    merged = true;
                    result.push(merged_range);
                    if i == 0 {
                        // println!("newvector: {:?}", result);
                    }
                }
            }
            if merged == false {
                break;
            }
            println!("result len: {:?}", result.len());
            println!("ranges len: {:?}", ranges.len());
            *ranges = std::mem::take(&mut result);
        }
    }

    pub fn vec_range(ranges: Vec<&str>) -> Vec<(u64, u64)> {
        let mut output = Vec::<(u64, u64)>::new();
        for i in ranges {
            let range: Vec<&str> = i.split('-').collect();
            let Ok(begin) = range[0].parse::<u64>() else {
                panic!("error begin!?");
            };
            let Ok(end) = range[1].parse::<u64>() else {
                panic!("error end!?");
            };
            if end <= begin {
                continue;
            }
            let range = (begin, end);
            output.push(range);
        }
        optimize_range(&mut output);
        output
    }

    pub fn in_range(ranges: &Vec<(u64, u64)>, value: u64) -> Option<()> {
        for range in ranges {
            //range is always sorted (1,2,3..99,100 always in order)
            let slice = range.0..=range.1;
            if slice.contains(&value) {
                return Some(());
            }
            // for i in range.0..=range.1 {
            //     // println!("{i} == {value}");
            //     if i == value {
            //         // println!("true!");
            //         return Some(());
            //     }
            // }
        }
        // println!("no match");
        None
    }

    pub fn split_range_value(string: &str) -> (Vec<&str>, Vec<u64>){
        let lines: Vec<&str> = string.lines().collect();
        let mut strings = Vec::new();
        let mut values_str = false;
        let mut output_vec = Vec::new();
        for value in lines {
            if value.is_empty() {
                values_str = true;
                continue;
            }
            if values_str {
                let Ok(val) = value.parse::<u64>() else {
                    panic!("val didn't get value!?");
                };
                output_vec.push(val);
                continue;
            }
            strings.push(value);
        }
        (strings, output_vec)
    }

    pub fn all_valid(ranges: &Vec<(u64, u64)>) -> u64 {
        let mut output = 0;
        for range in ranges {
            //[12-18] = 18-12 + 1 = 7
            //12,13,14,15,16,17,18 = 7
            // output += end - begin + 1;
            let slice = range.0..=range.1;
            output += slice.count() as u64;
        }
        output
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn ranges() {
            //check different ranges
            let ranges = vec!["1-5", "10-18", "25-100"];
            let ranges = vec_range(ranges);

            let exp_outut = vec![(1, 5), (10, 18), (25, 100)];
            for i in 0..exp_outut.len() {
                let range = ranges[i];
                let exp = exp_outut[i];
                assert_eq!(range, exp, "ranges are different");
            }

            //check if combine also works
            let ranges = vec!["1-5", "15-40", "2-8", "11-20", "10-41", "100-300"];
            let ranges = vec_range(ranges);
            let exp_output = [(1, 8), (10, 41), (100, 300)];
            for i in 0..exp_output.len() {
                assert_eq!(exp_output.iter().find(|&x| x == &ranges[i]), Some(&ranges[i]), "can't find range");
            }
        }

        #[test]
        fn invalid_range() {
            //check for invalid ranges
            let ranges = vec!["1-5", "100-30"];
            let ranges = vec_range(ranges);
            let exp_output = [(1, 5)];
            for i in 0..exp_output.len() {
                let range = ranges[i];
                let exp = exp_output[i];
                assert_eq!(range, exp, "ranges are different when combining");
            }
        }

        #[test]
        fn valid_in_range() {
            let ranges = vec!["3-5", "10-14", "16-20", "12-18"];
            let ranges = vec_range(ranges);
            let check_val = vec![1, 5, 8, 11, 17, 32];
            let exp_output = [None, Some(()), None, Some(()), Some(()), None];
            for i in 0..check_val.len() {
                let output = in_range(&ranges, check_val[i]);
                assert_eq!(output, exp_output[i], "valid isn't in range or should be in range");
            }
        }

        #[test]
        fn split_val() {
            let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
            let (ranges, values) = split_range_value(input);
            let ranges = vec_range(ranges);
            let exp_output = [None, Some(()), None, Some(()), Some(()), None];
            for i in 0..exp_output.len() {
                let output = in_range(&ranges, values[i]);
                assert_eq!(output, exp_output[i], "output isn't equal");
            }
        }

        #[test]
        fn optimize_range() {
            let ranges = vec!["3-5", "10-14", "16-20", "12-18"];
            let ranges = vec_range(ranges);
            let exp_output = [(3, 5), (10, 20)];
            for i in 0..exp_output.len() {
                assert_eq!(exp_output.iter().find(|&x| x == &ranges[i]), Some(&ranges[i]), "can't find range");
            }
        }

        #[test]
        fn sum_range() {
            let input = "3-5
10-14
16-20
12-18
9-21


";
//[3, 5], [9, 21]
            let (ranges, _) = split_range_value(input);
            let ranges = vec_range(ranges);
            let exp_output = 16;
            println!("{:?}",ranges);
            assert_eq!(all_valid(&ranges), exp_output, "sum of valid not equal");

            let input = "7-10
13-15
10-14
16-20
12-18
9-21


";
//[7,21]
            let (ranges, _) = split_range_value(input);
            let ranges = vec_range(ranges);
            let sum = all_valid(&ranges);
            //manualy count al values in range
            let (begin, end) = ranges[0];
            let exp_sum = begin..=end;
            let exp_sum = exp_sum.count();
            assert_eq!(sum, exp_sum as u64);

        }
    }
}

#[cfg(test)]
mod test {

    use super::{*, id_range::{split_range_value, vec_range, in_range}};

    #[test]
    fn read_input () {
        let content = fs::read_to_string("test_input.txt").expect("expect a file");
        let (ranges, values) = split_range_value(&content);
        let ranges = vec_range(ranges);
        let exp_output = [None, Some(()), None, Some(()), Some(()), None];
        for i in 0..exp_output.len() {
            let output = in_range(&ranges, values[i]);
            assert_eq!(output, exp_output[i], "output isn't equal at i= {}", i);
        }
    }
}