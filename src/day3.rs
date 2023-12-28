use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
}
#[derive(Debug)]
struct Number {
    value: u32,
    positions: Vec<Pos>,
}

fn locale_numbers(file_lines: &Vec<&str>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for (i, line) in file_lines.iter().enumerate() {
        let mut number_positions: Vec<Pos> = Vec::new();
        let mut number_str = "".to_string();
        for (j, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                number_str = number_str + &character.to_string();
                number_positions.push(Pos {
                    y: i as i32,
                    x: j as i32,
                })
            } else {
                if number_str != "" {
                    let number = Number {
                        value: number_str.parse().unwrap_or_default(),
                        positions: number_positions,
                    };
                    numbers.push(number);

                    number_positions = Vec::new();
                    number_str = "".to_string();
                }
            }
        }
        if number_str != "" {
            let number = Number {
                value: number_str.parse().unwrap_or_default(),
                positions: number_positions,
            };
            numbers.push(number);
        }
    }

    numbers
}

fn is_number_in_valid_position(number: &Number, valid_positions: &HashSet<Pos>) -> bool {
    for number_position in &number.positions {
        if valid_positions.contains(&number_position) {
            return true;
        }
    }

    return false;
}

#[allow(dead_code)]
fn main() {
    let mut valid_positions: HashSet<Pos> = HashSet::new();

    // let filename = "src/day3_example.txt";
    // let filename = "src/day3_test.txt";
    let filename = "src/day3_input.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();

    let symbols: Vec<char> = vec!['*', '#', '+', '$', '=', '@', '/', '%', '-', '&'];

    for (i, line) in file_lines.iter().enumerate() {
        for (j, character) in line.chars().enumerate() {
            let y = i as i32;
            let x = j as i32;
            if symbols.contains(&character) {
                valid_positions.insert(Pos { y: y - 1, x: x - 1 });
                valid_positions.insert(Pos { y: y - 1, x });
                valid_positions.insert(Pos { y: y - 1, x: x + 1 });

                valid_positions.insert(Pos { y, x: x - 1 });
                valid_positions.insert(Pos { y, x: x + 1 });

                valid_positions.insert(Pos { y: y + 1, x: x - 1 });
                valid_positions.insert(Pos { y: y + 1, x });
                valid_positions.insert(Pos { y: y + 1, x: x + 1 });
            }
        }
    }

    let numbers = locale_numbers(&file_lines);

    let mut sum = 0;
    for number in numbers.iter() {
        if is_number_in_valid_position(number, &valid_positions) {
            sum = sum + number.value;
        }
    }

    println!("{:?}", sum)

    //    println!("numbers: {:?}", numbers);
    //    println!("file_lines: {:?}", file_lines);
    //    println!("valid_positions: {:?}", valid_positions);
}

fn part2() {
    // 1 - find where the gears are located
    // 2 - Iterate over all the gears to find the adjacent numbers
}
