use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Card {
    numbers: Vec<u32>,
    winning_numbers: HashSet<u32>,
}

#[allow(dead_code)]
fn main() {
    // let filename = "src/day3_example.txt";
    let filename = "src/day4_input.txt";
    // let filename = "src/day4_example.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();

    let mut cards: Vec<Card> = vec![];
    for line in file_lines.iter() {
        let valid_line_data: &str = line.split(":").collect::<Vec<&str>>()[1].trim();
        let splitted_valid_line_data: Vec<&str> = valid_line_data.split("|").collect();

        let mut numbers: Vec<u32> = Vec::new();
        let mut winning_numbers: HashSet<u32> = HashSet::new();

        for winning_number_str in splitted_valid_line_data[0].trim().split_whitespace() {
            winning_numbers.insert(winning_number_str.parse().unwrap());
        }

        for number_str in splitted_valid_line_data[1].trim().split_whitespace() {
            numbers.push(number_str.parse().unwrap());
        }

        cards.push(Card {
            numbers,
            winning_numbers,
        })
    }

    println!("Cards {:?}", cards);

    let mut sum: i32 = 0;
    for card in cards {
        let mut counter = 0;
        for number in card.numbers {
            if card.winning_numbers.contains(&number) {
                counter = counter + 1;
            }
        }
        if counter != 0 {
            sum = sum + (2 as i32).pow(counter - 1);
            println!("counter: {:?}, result: {:?}", counter, (2 as i32).pow(counter - 1))
        }
    }

    println!("Part 1 Result: {:?}", sum)
}
