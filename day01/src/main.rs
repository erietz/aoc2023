use std::fs;

const INPUT_FILE: &str = "./input.txt";


fn main() {
    // part1();
    part2();
}


fn part1() {
    let contents = fs::read_to_string(INPUT_FILE).expect("coudn't read file");
    let mut total: i32 = 0;

    for line in contents.split("\n") {
        let mut first_digit: char = 'X';
        let mut last_digit: char = 'X';

        for char in line.chars() {
            if char.is_digit(10) {
                first_digit = char;
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_digit(10) {
                last_digit = char;
                break;
            }
        }


        if first_digit != 'X' && last_digit != 'X' {
            let num = String::from_iter([first_digit, last_digit]);
            total += num.to_string().parse::<i32>().unwrap();
        }
    }

    println!("The sum is {}", total);
}


fn part2() {
    let contents = fs::read_to_string(INPUT_FILE).expect("coudn't read file");
    let mut total: i32 = 0;

    for line in contents.split("\n") {
        if line.len() > 0 {
            total += parse_line(line);
        }
    }

    println!("The sum is {}", total);
}

struct Occurance {
    index: usize,
    value: char,
}

fn to_normalized_digit(num_str: &str) -> char {
    match num_str {
        "1" => '1',
        "2" => '2',
        "3" => '3',
        "4" => '4',
        "5" => '5',
        "6" => '6',
        "7" => '7',
        "8" => '8',
        "9" => '9',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("nothing matched"),
    }
}


fn parse_line(line: &str) -> i32 {
    let digits = [
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut occurances: Vec<Occurance> = Vec::new();
    for digit in digits {
        for (i, m) in line.match_indices(digit) {
            let occurance = Occurance {
                index: i,
                value: to_normalized_digit(&m.to_string()),
            };
            occurances.push(occurance);
        }
    }


    let mut min_index = usize::MAX;
    let mut max_index = 0;
    let mut min_i = 0;
    let mut max_i = 0;
    for (i, occurance) in occurances.iter().enumerate() {
        if occurance.index <= min_index {
            min_index = occurance.index;
            min_i = i;
        }
        if occurance.index >= max_index {
            max_index = occurance.index;
            max_i = i;
        }
    }

    let left = &occurances[min_i].value;
    let right = &occurances[max_i].value;

    let digits = String::from_iter([left, right]);
    let num = digits.to_string().parse::<i32>().unwrap();

    return num;
}
