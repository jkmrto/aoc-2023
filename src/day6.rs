use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
struct Race {
    time: i32,
    distance: i32,
}

#[allow(dead_code)]
fn main() {
    // let filename = "src/day6_example.txt";
    let filename = "src/day6_input.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();
    let times: Vec<i32> = file_lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|int_str| int_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let distances: Vec<i32> = file_lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|int_str| int_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut races: Vec<Race> = vec![];
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
        println!("index: {:?}", i)
    }

    println!("races: {:?}", races);

    let mut multiplier = 1;
    for race in races {
        println!("\nrace: {:?}", race);
        let mut counter = 0;
        for time_hold in 1..race.time {
            let distance = (race.time - time_hold) * time_hold;
            if distance > race.distance {
                counter = counter + 1;
            }
            // println!("time_hold: {:?}", distance)
        }
        multiplier = multiplier * counter;
        println!("counter: {:?}", counter)
    }

    println!("result: {:?}", multiplier)
}
