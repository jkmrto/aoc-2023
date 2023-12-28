use std::fs;

#[derive(Debug)]
struct Set {
    green: u32,
    blue: u32,
    red: u32,
}

#[derive(Debug)]
struct Game {
    index: u32,
    sets: Vec<Set>,
}

fn parse_set_cubes(sets_str: &str) -> Set {
    let mut set_cubes: Set = Set {
        green: 0,
        red: 0,
        blue: 0,
    };

    let set_cubes_str: Vec<&str> = sets_str.split(",").collect();

    for cube in set_cubes_str.iter() {
        let trimmed_cube = cube.trim().to_string();
        let parsed_cube: Vec<&str> = trimmed_cube.split(" ").collect();
        let color = parsed_cube[1];
        let n_balls = parsed_cube[0];
        if color == "green" {
            set_cubes.green = n_balls.parse().unwrap_or_default();
        }

        if color == "red" {
            set_cubes.red = n_balls.parse().unwrap_or_default();
        }

        if color == "blue" {
            set_cubes.blue = n_balls.parse().unwrap_or_default();
        }
    }

    set_cubes
}

fn parse_games(file_content: &str) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];

    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();
    for line in file_lines.iter() {
        let line_splitted: Vec<&str> = line.split(":").collect();
        let index_splitted: Vec<&str> = line_splitted[0].split(" ").collect();
        let game_index: u32 = index_splitted[1].parse().unwrap_or_default();

        let mut game: Game = Game {
            index: game_index,
            sets: Vec::<Set>::new(),
        };

        let game_sets_str: Vec<&str> = line_splitted[1].split(";").collect();
        for sets_str in game_sets_str.iter() {
            let set_cubes = parse_set_cubes(sets_str);
            game.sets.push(set_cubes);
        }

        games.push(game);
    }

    games
}

fn is_a_valid_game(game: &Game) -> bool {
    let balls_limit: Set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    for set in game.sets.iter() {
        if balls_limit.green < set.green {
            return false;
        }

        if balls_limit.red < set.red {
            return false;
        }

        if balls_limit.blue < set.blue {
            return false;
        }
    }

    true
}

fn min_required_set_of_cubes(game: &Game) -> Set {
    let mut min_set_cubes: Set = Set {
        green: 0,
        red: 0,
        blue: 0,
    };

    for set in game.sets.iter() {
        if min_set_cubes.green < set.green {
            min_set_cubes.green = set.green;
        }

        if min_set_cubes.red < set.red {
            min_set_cubes.red = set.red;
        }

        if min_set_cubes.blue < set.blue {
            min_set_cubes.blue = set.blue;
        }
    }

    min_set_cubes
}

fn part1(games: &Vec<Game>) -> u32 {
    let filtered_games: Vec<&Game> = games.iter().filter(|game| is_a_valid_game(game)).collect();

    let result: u32 = filtered_games
        .iter()
        .map(|game| game.index)
        .reduce(|acc, index| index + acc)
        .unwrap_or_default();

    result
}

fn part2(games: &Vec<Game>) -> u32 {
    let min_required_cubes_per_game: Vec<Set> = games.iter().map(|game| min_required_set_of_cubes(game)).collect();

    let result: u32 = min_required_cubes_per_game
        .iter()
        .map(|min_required_cubes| min_required_cubes.green * min_required_cubes.blue * min_required_cubes.red)
        .reduce(|acc, index| index + acc)
        .unwrap_or_default();

    result
}

#[allow(dead_code)]
fn main() {
    // let filename = "src/day2_example.txt";
    let filename = "src/day2_input.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let games = parse_games(&file_content);

    println!("Part1 result: {:?}", part1(&games));
    println!("Part2 result: {:?}", part2(&games));
}
