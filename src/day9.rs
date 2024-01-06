use std::fs;

#[allow(dead_code)]
fn main() {
    let filename = "src/day9_input.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();
    let input: Vec<Vec<i32>> = file_lines
        .iter()
        .map(|file_line| {
            file_line
                .split_whitespace()
                .map(|number_str| number_str.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut total_sum: i32 = 0;
    for input_line in &input {
        let mut container: Vec<Vec<i32>> = vec![];
        container.push(input_line.clone());
        let container = build_deltas(input_line, container);

        let mut sum: i32 = 0;
        for deltas in &container {
            sum = sum + deltas[deltas.len() - 1];
        }

        total_sum = total_sum + sum;
        println!("Container: {:?}, sum: {:?}", container, sum);
    }

    println!("Total sum {:?}", total_sum);
}

fn build_deltas(input: &Vec<i32>, mut container: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut deltas: Vec<i32> = vec![];
    for i in 0..input.len() - 1 {
        deltas.push(input[i + 1] - input[i])
    }

    //    println!("Deltas: {:?}, Are all zeros?: {:?}", &deltas, are_all_zeros(&deltas));

    if are_all_zeros(&deltas) {
        return container;
    } else {
        container.push(deltas.clone());
        return build_deltas(&deltas, container);
    }
}

fn are_all_zeros(vector: &Vec<i32>) -> bool {
    return vector
        .iter()
        .filter(|&&number| number == 0)
        .collect::<Vec<&i32>>()
        .len()
        == vector.len();
}
