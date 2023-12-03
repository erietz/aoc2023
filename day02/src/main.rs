use std::fs;

const INPUT_FILE: &str = "./input.txt";

#[derive(Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>
}


fn main() {
    part1();
    // part2();
}


#[allow(dead_code)]
fn part1() {
    // let contents = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    let contents = fs::read_to_string(INPUT_FILE).unwrap();
    let contents = contents.trim_end();

    let mut total = 0;
    for line in contents.split("\n") {
        let game = parse_game(line);
        // println!("{:?}", game);

        let is_possible = game.rounds.iter().fold(true, |acc, curr| {
            if curr.red <= 12 && curr.green <= 13 && curr.blue <= 14 {
                acc && true
            } else {
                false
            }
        });

        if is_possible {
            total += game.id;
        }
    }

    println!("The total is {}", total);
}


#[allow(dead_code)]
fn part2() {
    let contents = fs::read_to_string(INPUT_FILE).expect("Failed to read input file");

    let mut total = 0;
    for line in contents.trim_end().split("\n") {
        let game = parse_game(line);
        // println!("{:?}", game);

        let red = game.rounds.iter().map(|r| r.red).max().unwrap();
        let green = game.rounds.iter().map(|r| r.green).max().unwrap();
        let blue = game.rounds.iter().map(|r| r.blue).max().unwrap();
        total += red * green * blue;
    }

    println!("The total is {}", total);
}


fn parse_game(line: &str) -> Game {
    let mut rounds: Vec<Round> = Vec::new();

    let parts: Vec<&str> = line.split(":").collect();
    let id = parts[0].replace("Game ", "").parse::<i32>().unwrap();

    for round in parts[1].split(";").map(|x| x.trim()) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cubes: Vec<&str> = round.split(",").map(|x| x.trim()).collect();
        for cube in cubes {
            let tmp: Vec<&str> = cube.split(" ").map(|x| x.trim()).collect();
            let num_cube = tmp[0].parse::<i32>().unwrap();
            let color_cube = tmp[1];

            match color_cube {
                "red" => red = num_cube,
                "green" => green = num_cube,
                "blue" => blue = num_cube,
                _ => panic!("unknown color"),
            }
        }

        let round = Round { red, blue, green, };
        rounds.push(round);
    }

    Game{ id, rounds, }
}
