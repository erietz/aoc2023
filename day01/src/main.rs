use std::fs;

const INPUT_FILE: &str = "./input.txt";


fn main() {
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
