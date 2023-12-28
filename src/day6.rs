use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
struct Race {
    time: i64,
    distance: i64,
}

fn part1(times: &Vec<i64>, distances: &Vec<i64>) {
    let mut races: Vec<Race> = vec![];
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }

    let mut multiplier = 1;
    for race in races {
        let mut counter = 0;
        for time_hold in 1..race.time {
            let distance = (race.time - time_hold) * time_hold;
            if distance > race.distance {
                counter = counter + 1;
            }
        }
        multiplier = multiplier * counter;
    }

    println!("Part 1 -> {:?}", multiplier)
}

fn merge_in_one_integer(integers: &Vec<i64>) -> i64 {
    integers
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn part2(times: &Vec<i64>, distances: &Vec<i64>) {
    let distance: i64 = merge_in_one_integer(distances);
    let time: i64 = merge_in_one_integer(times);

    let mut counter = 0;
    for time_hold in 1..time {
        let race_distance = (time - time_hold) * time_hold;
        if race_distance > distance {
            counter = counter + 1;
        }
    }

    println!("Part 2 -> {:?}", counter);
}

#[allow(dead_code)]
fn main() {
    // let filename = "src/day6_example.txt";
    let filename = "src/day6_input.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();
    let times: Vec<i64> = file_lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|int_str| int_str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let distances: Vec<i64> = file_lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|int_str| int_str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    part1(&times, &distances);
    part2(&times, &distances);
}
