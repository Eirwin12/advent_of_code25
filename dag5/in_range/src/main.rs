use std::fs;

fn main() {
    println!("Hello, world!");
}

mod id_range {
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
        output
    }

    pub fn in_range(ranges: &Vec<Range>, value: u64) -> Option<()> {
        for range in ranges {
            for i in range.begin..=range.end {
                if i == value {
                    return Some(());
                }
            }
        }
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
            for i in check_val {
                let output = in_range(&ranges, i);
                assert_eq!(output, exp_output[1], "valid isn't in range or should be in range");
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
    }
}

#[cfg(test)]
mod test {

    use super::{*, id_range::{split_range_value, vec_range, in_range}};

    #[test]
    fn read_input () {
        let content = fs::read_to_string("../test_input.txt").expect("expect a file");
        let (ranges, values) = split_range_value(&content);
        let ranges = vec_range(ranges);
        let exp_output = [None, Some(()), None, Some(()), Some(()), None];
        for i in 0..exp_output.len() {
            let output = in_range(&ranges, values[i]);
            assert_eq!(output, exp_output[i], "output isn't equal");
        }
    }
}