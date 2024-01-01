use std::collections::HashMap;
use std::fs;

#[derive(PartialOrd, Debug, Eq, Hash, PartialEq)]
struct Hand {
    cards: String,
    amount: i64,
    hand_type: String,
    rank_by_cards: i64,
    cards_value: i64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Implement your custjm comparison logic here
        if self.rank_by_cards == other.rank_by_cards {
            self.cards_value.cmp(&other.cards_value)
        } else {
            self.rank_by_cards.cmp(&other.rank_by_cards)
        }
    }
}

fn part2(file_lines: &Vec<&str>) {
    let mut char_to_value: HashMap<char, i64> = HashMap::new();
    char_to_value.insert('A', 13);
    char_to_value.insert('K', 12);
    char_to_value.insert('Q', 11);
    char_to_value.insert('T', 10);
    char_to_value.insert('9', 9);
    char_to_value.insert('8', 8);
    char_to_value.insert('7', 7);
    char_to_value.insert('6', 6);
    char_to_value.insert('5', 5);
    char_to_value.insert('4', 4);
    char_to_value.insert('3', 3);
    char_to_value.insert('2', 2);
    char_to_value.insert('J', 1);

    let mut hands: Vec<Hand> = vec![];
    for line in file_lines {
        let cards = line.split(" ").collect::<Vec<&str>>()[0].to_string();

        let mut char_counts: HashMap<char, i32> = HashMap::new();
        for c in cards.chars() {
            let counter = char_counts.entry(c).or_insert(0 as i32);
            *counter += 1 as i32;
        }

        let mut values: Vec<i32> = char_counts.values().cloned().collect();
        values.sort();
        let values_str: String = values.iter().map(|&x| x.to_string()).collect();

        let n_jokers = char_counts.get(&'J').unwrap_or(&0);

        let pointer_str: &str = &values_str;
        let rank_by_cards = match pointer_str {
            "11111" => {
                if n_jokers == &1 {
                    2
                } else if n_jokers == &0 {
                    1
                } else {
                    panic!("This is not expected {:?}", pointer_str)
                }
            }
            "1112" => {
                if n_jokers == &2 {
                    4
                } else if n_jokers == &1 {
                    4
                } else if n_jokers == &0 {
                    2
                } else {
                    panic!("This is not expected {:?}", pointer_str)
                }
            }
            "122" => {
                if n_jokers == &2 {
                    6
                } else if n_jokers == &1 {
                    5
                } else if n_jokers == &0 {
                    3
                } else {
                    panic!("This is not expected {:?}", pointer_str)
                }
            }
            "113" => {
                if n_jokers == &3 {
                    6
                } else if n_jokers == &1 {
                    6
                } else if n_jokers == &0 {
                    4
                } else {
                    panic!("This is not expected {:?}", pointer_str)
                }
            }
            "23" => {
                if n_jokers == &3 {
                    7
                } else if n_jokers == &2 {
                    7
                } else if n_jokers == &0 {
                    5
                } else {
                    panic!("This is not expected {:?}", pointer_str)
                }
            }
            "14" => {
                if n_jokers == &4 {
                    7
                } else if n_jokers == &1 {
                    7
                } else if n_jokers == &0 {
                    6
                } else {
                    panic!("This is not expected {:?}", pointer_str)
                }
            }

            "5" => 7,
            _ => panic!("ERROR: Unsupported numbers: {:?}", pointer_str),
        };

        let mut cards_value: i64 = 0;
        for (index, char) in cards.char_indices() {
            let base = 13 as i64;
            let exponent: i64 = 5 - 1 - index as i64;
            cards_value = cards_value + char_to_value[&char] * base.pow(exponent as u32);
        }

        let hand = Hand {
            cards: line.split(" ").collect::<Vec<&str>>()[0].to_string(),
            amount: line.split(" ").collect::<Vec<&str>>()[1].to_string().parse().unwrap(),
            hand_type: values_str.to_string(),
            cards_value,
            rank_by_cards,
        };

        hands.push(hand);
    }

    hands.sort_by(|a, b| a.cmp(b));

    let hands_amount: Vec<i64> = hands.iter().map(|hand| hand.amount).collect();

    let mut sum: i64 = 0;
    for (index, value) in hands_amount.iter().enumerate() {
        sum = sum + (1 + index as i64) * *value as i64;
    }

    println!("Part2 result: {:?}\n", sum);
}

fn part1(file_lines: &Vec<&str>) {
    let mut char_to_value: HashMap<char, i64> = HashMap::new();
    char_to_value.insert('A', 13);
    char_to_value.insert('K', 12);
    char_to_value.insert('Q', 11);
    char_to_value.insert('J', 10);
    char_to_value.insert('T', 9);
    char_to_value.insert('9', 8);
    char_to_value.insert('8', 7);
    char_to_value.insert('7', 6);
    char_to_value.insert('6', 5);
    char_to_value.insert('5', 4);
    char_to_value.insert('4', 3);
    char_to_value.insert('3', 2);
    char_to_value.insert('2', 1);

    let mut hands: Vec<Hand> = vec![];
    for line in file_lines {
        let cards = line.split(" ").collect::<Vec<&str>>()[0].to_string();

        let mut char_counts: HashMap<char, usize> = HashMap::new();
        for c in cards.chars() {
            let counter = char_counts.entry(c).or_insert(0);
            *counter += 1;
        }

        let mut values: Vec<usize> = char_counts.values().cloned().collect();
        values.sort();
        let values_str: String = values.iter().map(|&x| x.to_string()).collect();

        let pointer_str: &str = &values_str;
        let rank_by_cards = match pointer_str {
            "11111" => 1,
            "1112" => 2,
            "122" => 3,
            "113" => 4,
            "23" => 5,
            "14" => 6,
            "5" => 7,
            _ => panic!("ERROR: Unsupported numbers: {:?}", pointer_str),
        };

        let mut cards_value: i64 = 0;
        for (index, char) in cards.char_indices() {
            let base = 13 as i64;
            let exponent: i64 = 5 - 1 - index as i64;
            cards_value = cards_value + char_to_value[&char] * base.pow(exponent as u32);
        }

        let hand = Hand {
            cards: line.split(" ").collect::<Vec<&str>>()[0].to_string(),
            amount: line.split(" ").collect::<Vec<&str>>()[1].to_string().parse().unwrap(),
            hand_type: values_str.to_string(),
            cards_value,
            rank_by_cards,
        };

        hands.push(hand);
    }

    hands.sort_by(|a, b| a.cmp(b));

    let hands_amount: Vec<i64> = hands.iter().map(|hand| hand.amount).collect();

    let mut sum: i64 = 0;
    for (index, value) in hands_amount.iter().enumerate() {
        sum = sum + (1 + index as i64) * *value as i64;
    }

    println!("Part1 result: {:?}\n", sum);
}

#[allow(dead_code)]
fn main() {
    let filename = "src/day7_input.txt";
    // let filename = "src/day7_example.txt";
    // let filename = "src/day7_input.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();

    part1(&file_lines);
    part2(&file_lines)
}
