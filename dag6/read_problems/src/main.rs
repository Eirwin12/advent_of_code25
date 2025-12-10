use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("expect a file");
    // let problems;
    // match read_problem::read_collumns(&content) {
    //     Ok(vector) => problems = vector,
    //     Err(error) => panic!("panic with values: {:?}", error),
    // }
    // let Ok(sum) = read_problem::sum_problems(&problems) else {
    //     panic!("can't sum");
    // };
    // println!("sum is: {}", sum);

    let Some(problems) = read_problem::read_problem_p2(&content) else {
        panic!("panic with making problems");
    };
    let Ok(sum) = read_problem::sum_problems(&problems) else {
        panic!("panic with making problems");
    };
    println!("sum part 2 is: {}", sum);
}

mod read_problem {

    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Clone, Copy)]
    enum Operator {
        Sum,
        Mult,
        // Div, 
        // diff,
        NoOperator,
    }

    #[derive(Debug)]
    pub enum OperatorErr {
        NoErr,
        NoOperator,
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct Problem {
        operator: Operator,
        values: Vec<i64>,
    }

    impl Problem {
        fn new() -> Self {
            Problem { operator: Operator::NoOperator, values: Vec::with_capacity(2) }
        }
        fn add_operator(&mut self, operator: Operator) {
            self.operator = operator;
        }
        fn add_value(&mut self, value: i64) {
            self.values.push(value);
        }
        fn do_problem(&self) -> Option<i128> {
            let mut output: i128;
            if self.operator == Operator::Mult { output = 1; }
            else { output = 0; }
            for value in self.values.clone(){
                match self.operator {
                    Operator::Sum =>  output += value as i128,
                    Operator::Mult => output *= value as i128,
                    _ => return None,
                }
            }
            Some(output)
        }
    }

    //return index of problem array of problem and which number has problem or if operator has problem
    pub fn read_collumns(string: &str) ->Result<Vec<Problem>, (usize, usize, OperatorErr)>{
        let string: Vec<&str> = string.lines().collect();
        //example input:
        // 1 1 1
        // 1 1 1
        // + + +
        let mut problem = Vec::<Problem>::new();
        for i in 0..string.len() {
            //string has: 1 1 1 or + + +
            
            //split werkt zoals verwacht. Het haalt alleen 1 spatie weg
            //overige spaties worden dan 'empty strings'
            // println!("string: {:?}", string);
            
            let amount_lines = string.len();

            let string: Vec<&str> = string[i].split_whitespace().collect();
            println!("string: {:?}", string);
            for vector_index in 0..string.len() {
                // println!("i = {i}, j = {vector_index}");
                //there could be whitespace in values/operator
                // println!("index: {i}, found string: {}", string[i]);
                // println!("found string: {}", string);
                if i == 0 {
                    problem.push(Problem::new());
                    let Ok(value) = string[vector_index].parse::<i64>() else {
                        return Err((0, 0, OperatorErr::NoErr));
                    };
                    problem[vector_index].add_value(value);
                    continue;
                }
                //last value has the operator
                if i == (amount_lines-1) {
                    match string[vector_index] {
                        "+" => problem[vector_index].add_operator(Operator::Sum),
                        "*" => problem[vector_index].add_operator(Operator::Mult),
                        _   => return Err((i, vector_index, OperatorErr::NoOperator)),
                    }
                    continue;
                }
                let Ok(value) = string[vector_index].parse::<i64>() else {
                    return Err((i, vector_index, OperatorErr::NoErr));
                };
                problem[vector_index].add_value(value);
                println!("new vector: {:?}", problem[vector_index].values);
            }
        }
        Ok(problem)
    }

    fn read_str(string: u8) ->Option<u8> {
        //there exist no '0'
        match string {
            b' ' => Some(0),
            b'0' => Some(0),
            b'1' => Some(1),
            b'2' => Some(2),
            b'3' => Some(3),
            b'4' => Some(4),
            b'5' => Some(5),
            b'6' => Some(6),
            b'7' => Some(7),
            b'8' => Some(8),
            b'9' => Some(9),
            _ => None,
        }
    }

    fn read_op(string: u8) ->Result<Operator, OperatorErr> {
        //there exist no '0'
        match string {
            b'+' => Ok(Operator::Sum),
            b'*' => Ok(Operator::Mult),
            _ => Err(OperatorErr::NoOperator),
        }
    }

    fn check_collumn(strings: &Vec<&str>, index: usize, check: u8) -> bool {
        for digits in strings {
            if digits.as_bytes()[index] != check {
                return false;
            }
        }
        true
    }
    
    use std::collections::VecDeque;

    fn to_problem(string: &Vec<&str>, begin_i: usize, end_i: usize) -> Problem {
        let mut value_vector: Vec<i64> = Vec::new();
        let mut operator_col: Operator = Operator::NoOperator;
        for collumn in begin_i..end_i {
        let mut digit_vector: VecDeque::<u8> = VecDeque::new();
        //dont do operator
        println!("collumn: {collumn}");
        for row in 0..string.len()-1{
            println!("found: {}", String::from_utf8(vec![string[row].as_bytes()[collumn]]).unwrap());
            digit_vector.push_front(read_str(string[row].as_bytes()[collumn]).unwrap());
        }
        println!("digits: {:?}", digit_vector);
        value_vector.push(combine_vec(digit_vector));
        }
        for collumn in begin_i..end_i {
            let row = string.len()-1;
            if string[row].as_bytes()[collumn] == b' ' {
                continue;
            }
            match read_op(string[row].as_bytes()[collumn]) {
                Ok(op) =>operator_col = op,
                Err(_) => operator_col = Operator::NoOperator,
            }
            break;
        }
        Problem { operator: operator_col, values: value_vector}
    }

    // fn reverse_deque(mut deque: &mut VecDeque<u8>) -> VecDeque<u8>{
    //     let mut vector: Vec<&u8> = deque.iter().collect();
    //     vector.reverse();
    //     let mut deque: VecDeque<u8> = VecDeque::new();
    //     for i in vector {
    //         deque.push_back(*i);
    //     }
    //     deque
    // }

    fn combine_vec(mut digits: VecDeque<u8>) ->i64 {
        // digits.reverse();
        let mut output = 0_i64;
        while digits.contains(&0) {
            if digits[0] == 0 {
                digits.pop_front();
            }
            if digits.iter().last() == Some(&0) {
                digits.pop_back();
            }
        }
        for i in 0..digits.len() {
            output += digits[i] as i64 *(10_i64.pow(i as u32));
        }
        output
    }

    pub fn read_problem_p2(string: &str) ->Option<Vec<Problem>>{
        let mut output:Vec<Problem> = Vec::new();
        let string_line = string.lines().collect::<Vec<&str>>();
        //vec:
        //  1 next
        // 1  next
        //1   next
        //zoek dus naar de collomn waar alleen spaties is
        let mut last_str_index_col = 0_usize;
        //last character is always \n
        //every string of numbers are equally long
        for str_index_col in 0..&string_line[0].len()-1 {
            if check_collumn(&string_line, str_index_col, b' ') == false{
                continue;
            }
            let problem = to_problem(&string_line, last_str_index_col, str_index_col);
            println!("problem: {:?}", problem);
            output.push(problem);

            //next value is valid numbers
            last_str_index_col = str_index_col +1;
        }
        Some(output)
    }

    fn calc_problems(problems: &Vec<Problem>) -> Result<Vec<i128>, OperatorErr> {
        let mut solution = Vec::<i128>::with_capacity(problems.len());
        for problem in problems {
            match problem.do_problem() {
                Some(value) => solution.push(value),
                _ => return Err(OperatorErr::NoOperator),
            }
        }
        Ok(solution)
    }

    pub fn sum_problems(problem: &Vec<Problem>) -> Result<i128, OperatorErr> {
        let solution = calc_problems(problem)?;
        let mut output = 0;
        for i in solution {
            output += i;
        }
        Ok(output)
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn reading_collumns() {
            let input = "1  1  1  1
                               1  1  1  1
                               +  +  +  +";
            let output = read_collumns(input);
            match output {
                Ok(value) => {
                    let mut exp_output = Problem::new();
                    exp_output.add_operator(Operator::Sum);
                    exp_output.add_value(1);
                    exp_output.add_value(1);
                    for i in value {
                        assert_eq!(exp_output, i, "problem are not equal");
                    }
                }
                Err(error) => {
                    let (vector_index, problem_index, error) = error;
                    panic!("can't creat problem at problem {}, number {} and the operator {:?}", vector_index, problem_index, error);
                }
            }

            let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
            let output = read_collumns(input);
            match output {
                Ok(value) => {
                    let mut exp_output = Vec::new();
                    {
                        let mut new_problem = Problem::new();
                        new_problem.add_operator(Operator::Mult);
                        new_problem.add_value(123);
                        new_problem.add_value(45);
                        new_problem.add_value(6);
                        exp_output.push(new_problem);
                    }
                    {
                        let mut new_problem = Problem::new();
                        new_problem.add_operator(Operator::Sum);
                        new_problem.add_value(328);
                        new_problem.add_value(64);
                        new_problem.add_value(98);
                        exp_output.push(new_problem);
                    }
                    {
                        let mut new_problem = Problem::new();
                        new_problem.add_operator(Operator::Mult);
                        new_problem.add_value(51);
                        new_problem.add_value(387);
                        new_problem.add_value(215);
                        exp_output.push(new_problem);
                    }
                    {
                        let mut new_problem = Problem::new();
                        new_problem.add_operator(Operator::Sum);
                        new_problem.add_value(64);
                        new_problem.add_value(23);
                        new_problem.add_value(314);
                        exp_output.push(new_problem);
                    }
                    for i in 0..value.len() {
                        assert_eq!(exp_output[i], value[i], "problem are not equal");
                    }
                }
                Err(error) => {
                    let (vector_index, problem_index, error) = error;
                    panic!("can't create problem at problem {}, number {} and the operator {:?}", vector_index, problem_index, error);
                }
            }
        }
        #[test]
        fn calculate() {
            let mut problem: Vec<Problem> = Vec::new();
            let mut exp_output: Vec<i128> = Vec::new();
            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Sum);
                for _ in 0..3 {
                    new_problem.add_value(5);
                }
                new_problem.add_value(10);
                problem.push(new_problem);
                exp_output.push(25);
            }
            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Sum);
                for _ in 0..2 {
                    new_problem.add_value(8);
                }
                new_problem.add_value(12);
                problem.push(new_problem);
                exp_output.push(28);
            }
            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Sum);
                for _ in 0..9 {
                    new_problem.add_value(90);
                }
                new_problem.add_value(128);
                problem.push(new_problem);
                exp_output.push(938);
            }
            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Mult);
                for _ in 0..3 {
                    new_problem.add_value(5);
                }
                new_problem.add_value(8);
                problem.push(new_problem);
                exp_output.push(1000);
            }
            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Mult);
                for _ in 0..3 {
                    new_problem.add_value(98);
                }
                new_problem.add_value(876);
                problem.push(new_problem);
                exp_output.push(824_484_192);
            }

            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Mult);
                new_problem.add_value(123);
                new_problem.add_value(45);
                new_problem.add_value(6);
                problem.push(new_problem);
                exp_output.push(33210);
            }
            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Sum);
                new_problem.add_value(328);
                new_problem.add_value(64);
                new_problem.add_value(98);
                problem.push(new_problem);
                exp_output.push(490);
            }
            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Mult);
                new_problem.add_value(51);
                new_problem.add_value(387);
                new_problem.add_value(215);
                problem.push(new_problem);
                exp_output.push(4243455);
            }
            {
                let mut new_problem = Problem::new();
                new_problem.add_operator(Operator::Sum);
                new_problem.add_value(64);
                new_problem.add_value(23);
                new_problem.add_value(314);
                problem.push(new_problem);
                exp_output.push(401);
            }
            let Ok(solution) = calc_problems(&problem) else {
                panic!("got no valid operator");
            };
            for i in 0..solution.len() {
                println!("problem {i}");
                assert_eq!(solution[i], exp_output[i], "solution are not equal");
            }
        }

        #[test]
        fn calculate_sum() {
            let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
            let problems;
            match read_collumns(input) {
                Ok(vector) => problems = vector,
                _ => panic!("can't receive problems"),
            }
            let Ok(sums) = sum_problems(&problems) else {
                panic!("can't calculate problem")
            };
            assert_eq!(sums, 4277556);
        }
    }

        #[test]
        fn reading_collumns_p2() {
            let input = std::fs::read_to_string("part2_test.txt").expect("expect a file");
            let output = read_problem_p2(&input);
            let exp_output = vec![Problem{operator: Operator::Sum, values: vec![1, 1]},
                                                Problem{operator: Operator::Sum, values: vec![21, 1]},
                                                Problem{operator: Operator::Sum, values: vec![11,82, 3]},
                                                Problem{operator: Operator::Mult, values: vec![12, 51, 1]}];
            match output {
                Some(value) => {
                    for i in 0..value.len() {
                        println!("problem {i}");
                        println!("output: {:?}", value[i]);
                        assert_eq!(exp_output[i], value[i], "problem are not equal");
                    }
                }
                None => {
                    panic!("No problems could be made!");
                }
            }

            let input = std::fs::read_to_string("test_input.txt").expect("expect a file");
            let output = read_problem_p2(&input);
            match output {
                Some(value) => {
                    let exp_output = vec![Problem{operator: Operator::Mult, values: vec![1, 24, 356]},
                                                Problem{operator: Operator::Sum, values: vec![369, 248, 8]},
                                                Problem{operator: Operator::Mult, values: vec![32, 581, 175]},
                                                Problem{operator: Operator::Sum, values: vec![623, 431, 4]}];
                    for i in 0..value.len() {
                        assert_eq!(exp_output[i], value[i], "problem are not equal");
                    }
                }
                None => {
                    panic!("No problems could be made!");
                }
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input(){
        let content = fs::read_to_string("test_input.txt").expect("expect a file");
        let problems;
        match read_problem::read_collumns(&content) {
            Ok(vector) => problems = vector,
            _ => panic!("can't receive problems"),
        }
        let Ok(sums) = read_problem::sum_problems(&problems) else {
            panic!("can't calculate problem")
        };
        assert_eq!(sums, 4277556);
    }

    #[test]
    fn read_input_p2(){
        let content = fs::read_to_string("test_input.txt").expect("expect a file");
        let problems;
        match read_problem::read_collumns(&content) {
            Ok(vector) => problems = vector,
            _ => panic!("can't receive problems"),
        }
        let Ok(sums) = read_problem::sum_problems(&problems) else {
            panic!("can't calculate problem")
        };
        assert_eq!(sums, 4277556);
    }
}