use std::fs;
fn main() {
    let content = fs::read_to_string("test_input.txt").expect("path exist");
    let (_, point) = make_grid(&content);
    let (points, _) = make_biggest_square(point);
    println!("points for the bigges value is: {:?}", points);
}

fn make_grid(input: &str) -> (Vec<String>, Vec<(u16, u16)>) {
    //find smallest and biggest value
    let input: Vec<&str> = input.lines().collect();
    //(collumn, row)
    let mut bord: (u16, u16) = (0, 0);
    //collumn, row
    let mut points: Vec<(u16, u16)> = Vec::new();
    for string in input {
        let range: Vec<&str> = string.split(',').collect();
        let range: (u16, u16) = (range[0].parse().expect("not value begin"), range[1].parse().expect("not value end"));
        points.push(range);
        if range.0 > bord.0 {
            bord.0 = range.0;
        }
        if range.1 > bord.1 {
            bord.1 = range.1;
        }
    }
    let mut grid: Vec<String> = Vec::new();
    //make all the rows
    for i in 0..=bord.1 {
        let mut string = ".".repeat((bord.0+1).into());
        let mut on_bord = points.iter();
        loop {
            //search if there is point in row
            let Some(point) = on_bord.find(|&x| x.1 == i) else {
                break;
            };
            string.replace_range(&point.0.into()..=&point.0.into(), "#");
        }
        grid.push(string);
    }
    (grid, points)
}

//calcs the distance between points
//if this doesn't work, make actual opp. function
fn calc_square<const DIMENSION: usize>(point: ([u16;DIMENSION], [u16;DIMENSION])) -> u32 {
    let mut distance: u32 = 0;
    for i in 0..DIMENSION {
        let difference = point.0[i] as i32 - point.1[i] as i32;
        distance += (difference as i32).pow(2) as u32;
    }
    distance
}

fn make_biggest_square(all_points: Vec<(u16, u16)>) ->([(u16, u16); 2], u32) {
    //(collumn, row)
    let mut output: ([(u16, u16); 2], u32) = ([(0, 0);2], 0);
    for in_point in all_points.iter().enumerate() {
        //if both points are smaller than biggest val, dont check it
        //put highest point in the second slot
        for i in output.0.iter().rev().enumerate() {
            if i.1.0 > in_point.1.0 && i.1.1 > in_point.1.1 {
                continue;
            }
            let distance = calc_square(([in_point.1.0, in_point.1.1], [i.1.0, i.1.1]));
            if distance > output.1 {
                if i.1.0 > in_point.1.0 {
                    output = ([(i.1.0, i.1.1), (in_point.1.0, in_point.1.1)], distance);
                }
                else {
                    output = ([(in_point.1.0, in_point.1.1), (i.1.0, i.1.1)], distance);
                }
                break;
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bord() {
        let content = fs::read_to_string("test_input.txt").expect("path exist");
        let (grid, _) = make_grid(&content);
        for i in &grid {
            println!("{i}");
        }
        let exp_output= fs::read_to_string("test_grid.txt").expect("path exist");
        let exp_output: Vec<&str>  = exp_output.lines().collect();
        for i in grid.iter().enumerate() {
            println!("testing row {}", i.0);
            assert_eq!(exp_output[i.0], i.1);
        }
    }
    
    #[test]
    fn test_biggest_square() {
        let content = fs::read_to_string("test_input.txt").expect("path exist");
        let (_, points) = make_grid(&content);
        let exp_output: [(u16, u16); 2] = [(2, 5), (11, 1)];
        let (output, _) = make_biggest_square(points);
        for point in output.iter().enumerate() {
            assert_eq!(exp_output[point.0], *point.1, "the points aren't the same");
        }
    }
}