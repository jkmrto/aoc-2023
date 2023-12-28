use std::fs;

#[derive(Copy, Clone, Debug)]
struct Range {
    source: i64,
    range: i64,
    destiny: i64,
}

fn do_mapping_ranges(seed: i64, mapping_ranges: &Vec<Range>) -> i64 {
    for range in mapping_ranges {
        if seed >= range.source && seed < (range.source + range.range) {
            let delta = range.destiny - range.source;
            return seed + delta;
        }
    }
    return seed;
}

fn find_range(seed: i64, mapping_ranges: &Vec<Range>) -> Range {
    if seed < mapping_ranges[0].source {
        return Range {
            destiny: 1,
            source: 1,
            range: mapping_ranges[0].source,
        };
    }

    for i in 0..mapping_ranges.len() - 1 {
        let currenct_range_limit = mapping_ranges[i].source + mapping_ranges[i].range;
        if seed >= mapping_ranges[i].source && seed < currenct_range_limit {
            return mapping_ranges[i];
        }

        if seed >= currenct_range_limit && seed < mapping_ranges[i + 1].source {
            return Range {
                destiny: mapping_ranges[i].source + mapping_ranges[i].range,
                source: mapping_ranges[i].source + mapping_ranges[i].range,
                range: mapping_ranges[i + 1].source - (mapping_ranges[i].source - mapping_ranges[i].range),
            };
        }
    }

    let final_mapping_range = &mapping_ranges[mapping_ranges.len() - 1];
    let final_range_limit = final_mapping_range.source + final_mapping_range.range;
    if seed >= final_mapping_range.source && seed < final_range_limit {
        return mapping_ranges[mapping_ranges.len() - 1];
    }

    return Range {
        destiny: final_mapping_range.source + final_mapping_range.range,
        source: final_mapping_range.source + final_mapping_range.range,
        range: 10000000000,
    };
}

fn apply_mapping_range(ranges: Vec<[i64; 2]>, mapping_ranges: &Vec<Range>) -> Vec<[i64; 2]> {
    let mut ranges_output: Vec<[i64; 2]> = Vec::new();

    for range in ranges {
        let mut current_seed = range[0];
        loop {
            let found_range = find_range(current_seed, mapping_ranges);
            let delta = found_range.destiny - found_range.source;
            let current_seed_mapped = current_seed + delta;
            let current_range_final = range[0] + range[1];

            if current_range_final > found_range.source + found_range.range {
                ranges_output.push([
                    current_seed_mapped,
                    found_range.source + found_range.range - current_seed,
                ]);
                current_seed = found_range.source + found_range.range;
            } else if current_range_final == (found_range.source + found_range.range) {
                ranges_output.push([
                    current_seed_mapped,
                    (found_range.source + found_range.range) - current_seed,
                ]);
                break;
            } else if current_range_final < (found_range.source + found_range.range) {
                ranges_output.push([current_seed_mapped, current_range_final - current_seed]);
                break;
            }
        }
    }

    return ranges_output;
}

fn load_mappig_ranges(
    file_lines: &Vec<&str>,
) -> (
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
) {
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
    }

    // This is needed to ensure the range ar ordered by 'source' in Asc way.
    seed_to_soil.sort_by(|a, b| a.source.cmp(&b.source));
    soil_to_fertilizer.sort_by(|a, b| a.source.cmp(&b.source));
    fertilizer_to_water.sort_by(|a, b| a.source.cmp(&b.source));
    water_to_light.sort_by(|a, b| a.source.cmp(&b.source));
    light_to_temperature.sort_by(|a, b| a.source.cmp(&b.source));
    temperature_to_humidity.sort_by(|a, b| a.source.cmp(&b.source));
    humidity_to_location.sort_by(|a, b| a.source.cmp(&b.source));

    return (
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    );
}

#[allow(dead_code)]
fn main() {
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

    let (
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = load_mappig_ranges(&file_lines);

    // Part 1.
    let final_locations: Vec<i64> = seeds
        .iter()
        .map(|seed| {
            let soil = do_mapping_ranges(*seed, &seed_to_soil);
            let fertilizer = do_mapping_ranges(soil, &soil_to_fertilizer);
            let water = do_mapping_ranges(fertilizer, &fertilizer_to_water);
            let light = do_mapping_ranges(water, &water_to_light);
            let temperature = do_mapping_ranges(light, &light_to_temperature);
            let humidity = do_mapping_ranges(temperature, &temperature_to_humidity);
            let location = do_mapping_ranges(humidity, &humidity_to_location);

            return location;
        })
        .collect();

    println!(
        "Part 1 Result: {:?}",
        final_locations.iter().cloned().min().unwrap_or_default(),
    );

    // Part 2
    let list_seeds_chunks: Vec<[i64; 2]> = seeds.chunks(2).map(|v| [v[0], v[1]]).collect();
    let mut output = apply_mapping_range(list_seeds_chunks, &seed_to_soil);
    output = apply_mapping_range(output, &soil_to_fertilizer);
    output = apply_mapping_range(output, &fertilizer_to_water);
    output = apply_mapping_range(output, &water_to_light);
    output = apply_mapping_range(output, &light_to_temperature);
    output = apply_mapping_range(output, &temperature_to_humidity);
    output = apply_mapping_range(output, &humidity_to_location);
    println!(
        "Part 2 result: {:?}",
        output.iter().map(|x| x[0]).collect::<Vec<i64>>().iter().cloned().min()
    )
}
