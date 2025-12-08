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
    use std::slice;

    //parameters hoeven (bijna) nooit een reference te zijn
    //de type voor parameter kan wel reference zijn. 
    pub struct Range {
        begin: u64,
        end: u64,
    }

    impl Range {
        pub fn new(begin: u64, end: u64) -> Self {
            Range { begin, end }
        }
        pub fn get(&self) -> (&u64, &u64) {
            (&self.begin, &self.end)
        }
        pub fn clone(&self) -> Self {
            Range { begin: self.begin, end: self.end }
        }
    }

    fn add_in_range(ranges: &mut Vec<Range>, next_range: Range) {
        for i in 0..ranges.len() {
            let range = &mut ranges[i];
            //if next ranges begin smaller than now,
            //and next ranges end is between ranges begin and end
            //next_range begin --- range begin ---// 
            if next_range.begin < range.begin{
                //next_range begin --- range begin --- next_range end --- range end 
                if next_range.end >range.begin && next_range.end < range.end {
                    range.begin = next_range.begin;
                    return ();
                }
                //next_range begin --- range begin --- range end --- next_range end
                else if next_range.end > range.end {
                    range.begin = next_range.begin;
                    range.end = next_range.end;
                    return ();
                }
            }
            //range begin --- next_range begin --- range-end// 
            else if next_range.begin >=range.begin && next_range.begin < range.end {
                //range begin --- next_range begin --- range-end --- next_range end
                if next_range.end > range.end {
                    range.end = next_range.end;
                    return ();
                }
            }
        }
        //if it isn't in ranges, push val
        ranges.push(next_range);
    }

    fn optimize_range(ranges: Vec<Range>) -> Vec<Range> {
        let mut new_range = Vec::<Range>::with_capacity(ranges.len()-1);
        new_range.push(ranges[0].clone());
        
        for i in 1..ranges.len() {
            let range = ranges[i].clone();
            add_in_range(&mut new_range, range);
        }
        new_range
    }

    pub fn vec_range(ranges: Vec<&str>) -> Vec<Range> {
        let mut output = Vec::<Range>::new();
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
            let range = Range {begin, end};
            add_in_range(&mut output, range);
        }
        optimize_range(output)
    }

    pub fn in_range(ranges: &Vec<Range>, value: u64) -> Option<()> {
        for range in ranges {
            //range is always sorted (1,2,3..99,100 always in order)
            let slice = range.begin..=range.end;
            if slice.contains(&value) {
                return Some(());
            }
            // for i in range.begin..=range.end {
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

    pub fn all_valid(ranges: &Vec<Range>) -> u64 {
        let mut output = 0;
        for range in ranges {
            let (begin, end) = range.get();
            //[12-18] = 18-12 + 1 = 7
            //12,13,14,15,16,17,18 = 7
            // output += end - begin + 1;
            let slice = *begin..=*end;
            output = slice.count() as u64;
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

            let exp_outut = vec![Range::new(1, 5), Range::new(10, 18), Range::new(25, 100)];
            for i in 0..exp_outut.len() {
                let range = ranges[i].get();
                let exp = exp_outut[i].get();
                assert_eq!(range, exp, "ranges are different");
            }

            //check if combine also works
            let ranges = vec!["1-5", "15-40", "2-8", "11-20", "10-41", "100-300"];
            let ranges = vec_range(ranges);
            let exp_output = [Range::new(1, 8), Range::new(10, 41), Range::new(100, 300)];
            for i in 0..exp_output.len() {
                let range = ranges[i].get();
                let exp = exp_output[i].get();
                assert_eq!(range, exp, "ranges are different when combining");
            }
        }

        #[test]
        fn invalid_range() {
            //check for invalid ranges
            let ranges = vec!["1-5", "100-30"];
            let ranges = vec_range(ranges);
            let exp_output = [Range::new(1, 5)];
            for i in 0..exp_output.len() {
                let range = ranges[i].get();
                let exp = exp_output[i].get();
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
            let exp_output = [Range::new(3, 5), Range::new(10, 20)];
            for i in 0..exp_output.len() {
                let range = ranges[i].get();
                let exp = exp_output[i].get();
                assert_eq!(range, exp, "ranges are different when combining");
            }
        }

        #[test]
        fn sum_range() {
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
            let (ranges, _) = split_range_value(input);
            let ranges = vec_range(ranges);
            let exp_output = 14;
            assert_eq!(all_valid(&ranges), exp_output, "sum of valid not equal");

            let input = vec!["86306595096954-88466949626371"];
            let ranges = vec_range(input);
            let sum = all_valid(&ranges);
            //manualy count al values in range
            let (begin, end) = ranges[0].get();
            let exp_sum = *begin..=*end;
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
            assert_eq!(output, exp_output[i], "output isn't equal");
        }
    }
}