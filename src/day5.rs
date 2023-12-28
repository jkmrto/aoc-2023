use std::fs;

#[derive(Debug)]
struct Range {
    source: i64,
    range: i64,
    destiny: i64,
}

fn mapping_ranges(seed: i64, mapping_ranges: &Vec<Range>) -> i64 {
    //    println!("seed: {:?}", seed);
    for range in mapping_ranges {
        if seed >= range.source && seed < (range.source + range.range) {
            println!("destiny: {:?}, source: {:?}", range.destiny, range.source);
            let delta = range.destiny - range.source;
            //           println!("delta: {:?}", delta);
            return seed + delta;
        }
    }
    return seed;
}

#[allow(dead_code)]
fn main() {
    // let filename = "src/day3_example.txt";
    // let filename = "src/day5_example.txt";
    let filename = "src/day5_input.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();

    let seeds_line = file_lines[0];
    let seeds_str = seeds_line.split(": ").collect::<Vec<&str>>()[1];
    let seeds: Vec<i64> = seeds_str
        .split_whitespace()
        .map(|seed_str| seed_str.parse::<i64>().unwrap())
        .collect();

    let mut seed_to_soil: Vec<Range> = vec![];
    let mut soil_to_fertilizer: Vec<Range> = vec![];
    let mut fertilizer_to_water: Vec<Range> = vec![];
    let mut water_to_light: Vec<Range> = vec![];
    let mut light_to_temperature: Vec<Range> = vec![];
    let mut temperature_to_humidity: Vec<Range> = vec![];
    let mut humidity_to_location: Vec<Range> = vec![];
    let mut currenct_range_loaded: &mut Vec<Range> = &mut seed_to_soil;

    for line in file_lines.iter().skip(1) {
        if line == &"" {
            continue;
        }

        if line == &"seed-to-soil map:" {
            currenct_range_loaded = &mut seed_to_soil;
            continue;
        }

        if line == &"soil-to-fertilizer map:" {
            currenct_range_loaded = &mut soil_to_fertilizer;
            continue;
        }

        if line == &"fertilizer-to-water map:" {
            currenct_range_loaded = &mut fertilizer_to_water;
            continue;
        }

        if line == &"water-to-light map:" {
            currenct_range_loaded = &mut water_to_light;
            continue;
        }

        if line == &"light-to-temperature map:" {
            currenct_range_loaded = &mut light_to_temperature;
            continue;
        }

        if line == &"temperature-to-humidity map:" {
            currenct_range_loaded = &mut temperature_to_humidity;
            continue;
        }

        if line == &"humidity-to-location map:" {
            currenct_range_loaded = &mut humidity_to_location;
            continue;
        }

        let parsed_ranges: Vec<i64> = line
            .split_whitespace()
            .map(|int_str| int_str.parse::<i64>().unwrap())
            .collect();

        let range = Range {
            source: parsed_ranges[1],
            range: parsed_ranges[2],
            destiny: parsed_ranges[0],
        };

        currenct_range_loaded.push(range);

        if line == &"soil-to-fertilizer map:" {
            continue;
        }
    }
    println!("seed_to_soil: {:?}", seed_to_soil);
    println!("soil_to_fertilizer: {:?}", soil_to_fertilizer);
    println!("fertilizer_to_water: {:?}", fertilizer_to_water);
    println!("water_to_light: {:?}", water_to_light);
    println!("light_to_temperature: {:?}", light_to_temperature);
    println!("temperature_to_humidity: {:?}", temperature_to_humidity);
    println!("humidity_to_location: {:?}", humidity_to_location);

    let final_locations: Vec<i64> = seeds
        .iter()
        .map(|seed| {
            let soil = mapping_ranges(*seed, &seed_to_soil);
            let fertilizer = mapping_ranges(soil, &soil_to_fertilizer);
            let water = mapping_ranges(fertilizer, &fertilizer_to_water);
            let light = mapping_ranges(water, &water_to_light);
            let temperature = mapping_ranges(light, &light_to_temperature);
            let humidity = mapping_ranges(temperature, &temperature_to_humidity);
            let location = mapping_ranges(humidity, &humidity_to_location);

            //            println!("seed: {:?}", seed);
            //            println!("soil  {:?}", soil);
            //            println!("fertilizer: {:?}", fertilizer);
            //            println!("water : {:?}", water);
            //            println!("light: {:?}", light);
            //            println!("temperature : {:?}", temperature);
            //            println!("humidity: {:?}", humidity);
            //            println!("location : {:?}", location);

            return location;
        })
        .collect();

    println!(
        "Final locations: {:?}, min: {:?}",
        final_locations,
        final_locations.iter().cloned().min().unwrap_or_default(),
    )
}
