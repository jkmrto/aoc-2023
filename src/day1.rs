use std::fs;

#[allow(dead_code)]
fn main() {
    let filename = "src/day1_input.txt";
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.collect();

    let numbers_str = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers_digit = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut dynamic_vector: Vec<u32> = Vec::new();

    for line in vec.iter() {
        let mut first_digit: u32 = 0;
        let mut first_digit_pos: u32 = line.len() as u32;
        let mut last_digit: u32 = 0;
        let mut last_digit_pos: i32 = -1;

        for (index, _number) in numbers_str.iter().enumerate() {
            let pos_occurrences = find_all_occurrences(&line, [numbers_str[index], numbers_digit[index]].to_vec());
            if pos_occurrences.len() != 0 {
                if (pos_occurrences[0] as u32) < first_digit_pos {
                    first_digit_pos = pos_occurrences[0] as u32;
                    first_digit = (index + 1) as u32;
                }

                if (pos_occurrences[pos_occurrences.len() - 1] as i32) > last_digit_pos {
                    last_digit_pos = pos_occurrences[pos_occurrences.len() - 1] as i32;
                    last_digit = (index + 1) as u32;
                }
            }
        }
        dynamic_vector.push(first_digit * 10 + last_digit);
    }

    let sum: u32 = dynamic_vector.iter().sum();
    println!("Sum: {:?}", sum);
}

fn find_all_occurrences(main_string: &str, substrings: Vec<&str>) -> Vec<usize> {
    let mut occurrences = Vec::new();

    for substring in substrings {
        let mut start_index = 0;
        while let Some(index) = main_string[start_index..].find(substring) {
            let absolute_index = start_index + index;
            occurrences.push(absolute_index);
            start_index = absolute_index + substring.len();
        }
    }

    occurrences.sort();
    occurrences
}

#[allow(dead_code)]
fn first_challenge() {
    const RADIX: u32 = 10;

    let filename = "src/day1_input.txt";
    let contents = fs::read_to_string(filename).expect("error");
    let split = contents.split("\n");
    let vec: Vec<&str> = split.collect();

    let numbers = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut dynamic_vector: Vec<u32> = Vec::new();
    let mut first_digit;
    let mut last_digit;

    for line in vec.iter() {
        if line != &"" {
            first_digit = 0;
            last_digit = 0;

            for element in line.chars() {
                if numbers.contains(&element) {
                    if first_digit == 0 {
                        first_digit = element.to_digit(RADIX).unwrap();
                    }
                    last_digit = element.to_digit(RADIX).unwrap();
                }
            }

            dynamic_vector.push(first_digit * 10 + last_digit);
        }
    }

    let sum: u32 = dynamic_vector.iter().sum();
    println!("Sum: {:?}", sum);
}
