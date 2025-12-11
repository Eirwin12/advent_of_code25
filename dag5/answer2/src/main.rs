use std::fs;

fn parse_input(input: &str) -> (Box<[usize]>, Box<[usize]>) {
    let (ranges, nums) = input
        .trim()
        .split_once("\n\n")
        .expect("Unable to split input into ranges and ids");
    let ranges: Box<[(usize, usize)]> = ranges
        .trim()
        .lines()
        .map(|line| {
            let (lstr, rstr) = line.split_once("-").expect("Cannot parse range");
            (
                lstr.parse::<usize>().expect("Cannot parse integer"),
                rstr.parse::<usize>().expect("Cannot parse integer"),
            )
        })
        .collect();
    let n_ranges = ranges.len();
    let mut bounds: Box<[(usize, usize, bool)]> = ranges
        .iter()
        .enumerate()
        .flat_map(|(i, (l, r))| {
            std::iter::once((*l, i, true)).chain(std::iter::once((*r, i, false)))
        })
        .collect();
    bounds.sort_by(|(aval, ai, aflag), (bval, bi, bflag)| {
        (aval, ai, std::cmp::Reverse(aflag)).cmp(&(bval, bi, std::cmp::Reverse(bflag)))
    });
    let mut n_active = 0usize;
    let mut active = vec![false; n_ranges].into_boxed_slice();
    let mut combined = Vec::<usize>::with_capacity(n_ranges);
    for (val, i, starting) in bounds {
        if starting {
            if n_active == 0 {
                if let Some(last) = combined.last().copied()
                    && last == val
                {
                    // New range is starting exactly where the previous one
                    // stopped. So just ignore both and combine into one
                    // range.
                    combined.pop();
                } else {
                    combined.push(val);
                }
            }
            active[i] = true;
            n_active += 1;
        } else {
            let removed = std::mem::replace(&mut active[i], false);
            if removed {
                n_active -= 1;
                if n_active == 0 {
                    combined.push(val);
                }
            }
        }
    }
    (
        combined.into_boxed_slice(),
        nums.trim()
            .lines()
            .map(|num| num.parse::<usize>().expect("Cannot parse integer"))
            .collect(),
    )
}

fn solve(input: &str) -> (usize, usize) {
    let (ranges, nums) = parse_input(input);
    let part_1 = nums
        .iter()
        .filter(|n| match ranges.binary_search(n) {
            Ok(_) => true,
            Err(i) => i % 2 != 0,
        })
        .count();
    let part_2 = ranges
        .chunks_exact(2)
        .map(|pair| pair[1] + 1 - pair[0])
        .sum();
    (part_1, part_2)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum = solve(&input);
    println!("sum: {}", sum.1);
}