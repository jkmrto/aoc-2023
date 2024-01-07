use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
struct Pos {
    y: i32,
    x: i32,
}

#[allow(dead_code)]
fn main() {
    // let filename = "src/day10_input.txt";
    let filename = "src/day10_input.txt";

    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();

    let input: Vec<Vec<char>> = file_lines.iter().map(|file_line| file_line.chars().collect()).collect();

    println!("Input: {:?}", input);

    let mut start_pos = Pos { y: 0, x: 0 };
    for (y, input_line) in input.iter().enumerate() {
        for (x, &input_char) in input_line.iter().enumerate() {
            if input_char == 'S' {
                start_pos = Pos {
                    y: y as i32,
                    x: x as i32,
                }
            }
        }
    }

    let direction_mapping: HashMap<char, Vec<&str>> = [
        ('|', vec!["north", "south"]),
        ('-', vec!["east", "west"]),
        ('L', vec!["north", "east"]),
        ('J', vec!["north", "west"]),
        ('7', vec!["south", "west"]),
        ('F', vec!["south", "east"]),
        ('.', vec![]),
    ]
    .iter()
    .cloned()
    .collect();

    // println!("Start position: {:?}", start_pos);

    // println!("Direction mapping: {:?}", direction_mapping);

    let north_directions = direction_mapping
        .get(&input[(start_pos.y - 1) as usize][start_pos.x as usize])
        .unwrap();
    let south_directions = direction_mapping
        .get(&input[(start_pos.y + 1) as usize][start_pos.x as usize])
        .unwrap();
    let east_directions = direction_mapping
        .get(&input[start_pos.y as usize][(start_pos.x + 1) as usize])
        .unwrap();
    let west_directions = direction_mapping
        .get(&input[start_pos.y as usize][(start_pos.x - 1) as usize])
        .unwrap();

    // println!(
    //     "North: {:?}, south connected {:?}",
    //     north_directions,
    //     north_directions.contains(&"south")
    // );

    // println!(
    //     "South: {:?}, north connected {:?}",
    //     south_directions,
    //     south_directions.contains(&"north")
    // );

    // println!(
    //     "East: {:?}, west connected {:?}",
    //     east_directions,
    //     east_directions.contains(&"west")
    // );

    // println!(
    //     "West: {:?}, east_connected: {:?}",
    //     west_directions,
    //     west_directions.contains(&"east")
    // );

    let mut possible_initial_movements: Vec<&str> = vec![];

    if north_directions.contains(&"south") {
        possible_initial_movements.push("north")
    }

    if south_directions.contains(&"north") {
        possible_initial_movements.push("south")
    }

    if east_directions.contains(&"west") {
        possible_initial_movements.push("east")
    }

    if west_directions.contains(&"east") {
        possible_initial_movements.push("west")
    }

    println!("{:?}", possible_initial_movements);

    let mut movement_to_delta: HashMap<&str, Pos> = HashMap::new();
    movement_to_delta.insert("south", Pos { x: 0, y: 1 });
    movement_to_delta.insert("north", Pos { x: 0, y: -1 });
    movement_to_delta.insert("east", Pos { x: 1, y: 0 });
    movement_to_delta.insert("west", Pos { x: -1, y: 0 });

    let mut movement_to_income_direction: HashMap<&str, &str> = HashMap::new();
    movement_to_income_direction.insert("south", "north");
    movement_to_income_direction.insert("north", "south");
    movement_to_income_direction.insert("east", "west");
    movement_to_income_direction.insert("west", "east");

    let next_move = possible_initial_movements[0];
    let next_pos = Pos {
        y: start_pos.y + movement_to_delta.get(next_move).unwrap().y,
        x: start_pos.x + movement_to_delta.get(next_move).unwrap().x,
    };

    println!("next_move: {:?}, next_pos: {:?}", next_move, next_pos);

    let mut record: Vec<Pos> = vec![];
    let next_pos_income_move: &str = movement_to_income_direction.get(next_move).unwrap();

    let record = do_move(
        &direction_mapping,
        &movement_to_delta,
        &movement_to_income_direction,
        &input,
        &mut record,
        next_pos_income_move,
        next_pos,
    );

    let mut steps: Vec<usize> = vec![];
    for (index, _value) in record.iter().enumerate() {
        if record.len() - index > index {
            steps.push(index)
        } else {
            steps.push(record.len() - index)
        }
    }

    println!("steps: {:?}", steps.len() / 2);
}

fn do_move(
    direction_mapping: &HashMap<char, Vec<&str>>,
    movement_to_delta: &HashMap<&str, Pos>,
    movement_to_income_direction: &HashMap<&str, &str>,
    field: &Vec<Vec<char>>,
    record: &mut Vec<Pos>,
    income_direction: &str,
    pos: Pos,
) -> Vec<Pos> {
    record.push(Pos { x: pos.x, y: pos.y });
    if field[pos.y as usize][pos.x as usize] == 'S' {
        return record.to_vec();
    }

    let direction_options = direction_mapping.get(&field[pos.y as usize][pos.x as usize]).unwrap();
    println!(" direction_options: {:?}", direction_options);

    let direction_options_filtered: Vec<&&str> = direction_options
        .into_iter()
        .filter(|&direction| direction != &income_direction)
        .collect::<Vec<&&str>>();

    println!("direction_options_filtered: {:?}", direction_options_filtered);

    let next_move = direction_options_filtered[0];

    let next_pos = Pos {
        y: pos.y + movement_to_delta.get(next_move).unwrap().y,
        x: pos.x + movement_to_delta.get(next_move).unwrap().x,
    };

    let next_pos_income_move: &str = movement_to_income_direction.get(next_move).unwrap();
    do_move(
        direction_mapping,
        movement_to_delta,
        movement_to_income_direction,
        field,
        record,
        next_pos_income_move,
        next_pos,
    )
}
