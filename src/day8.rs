use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

fn do_step(label_to_node: &HashMap<String, Node>, instructions: Vec<char>, node: &Node, steps: i32) -> i32 {
    if node.label == "ZZZ" {
        return steps;
    }

    let instruction = instructions[(steps as usize) % instructions.len()];

    if instruction == 'L' {
        let next_node = label_to_node.get(&node.left).unwrap();
        return do_step(&label_to_node, instructions, next_node, steps + 1);
    } else if instruction == 'R' {
        let next_node = label_to_node.get(&node.right).unwrap();
        return do_step(&label_to_node, instructions, next_node, steps + 1);
    } else {
        panic!("This is not going well")
    }
}

fn do_step_part2(
    label_to_node: &HashMap<String, Node>,
    instructions: Vec<char>,
    node: &Node,
    steps: i32,
    mut visited_nodes: Vec<String>,
) -> i32 {
    if node.label.chars().nth(2).unwrap() == 'Z' {
        return steps;
    }

    if visited_nodes.contains(&node.label) {
        return 0;
    }

    let instruction = instructions[(steps as usize) % instructions.len()];

    println!("hello");
    if instruction == 'L' {
        println!("hello 11");
        let next_node = label_to_node.get(&node.left).unwrap();
        println!("next node: {:?}", next_node);
        visited_nodes.push(node.label.clone());
        return do_step_part2(label_to_node, instructions, next_node, steps + 1, visited_nodes);
    } else if instruction == 'R' {
        println!("hello 12");
        let next_node = label_to_node.get(&node.right).unwrap();
        println!("next node: {:?}", next_node);
        visited_nodes.push(node.label.clone());
        return do_step_part2(label_to_node, instructions, next_node, steps + 1, visited_nodes);
    } else {
        println!("hello 13");
        panic!("This is not going well")
    }
}

fn build_label_to_nodes(file_lines: Vec<&str>) -> HashMap<String, Node> {
    let mut label_to_node: HashMap<String, Node> = HashMap::new();

    let chars_to_remove: Vec<char> = vec!['(', ')'];
    for line in file_lines.iter().skip(1) {
        let splitted_line: Vec<&str> = line.split(" = ").collect::<Vec<&str>>();

        let filtered_string: String = splitted_line[1]
            .chars()
            .filter(|&c| !chars_to_remove.contains(&c))
            .collect();

        let left_right_nodes: Vec<&str> = filtered_string.split(", ").collect();

        let node = Node {
            label: splitted_line[0].to_string(),
            left: left_right_nodes[0].to_string(),
            right: left_right_nodes[1].to_string(),
        };

        label_to_node.insert(splitted_line[0].to_string(), node);
    }

    return label_to_node;
}

fn part1(file_lines: Vec<&str>, instructions: Vec<char>) {
    let label_to_node = build_label_to_nodes(file_lines);
    println!("Label to Node: {:?}", label_to_node);

    let steps = 0;
    let node = label_to_node.get("AAA").unwrap();

    let steps = do_step(&label_to_node, instructions, &node, steps);

    println!("Steps: {:?}", steps)
}

#[allow(dead_code)]
fn main() {
    // let filename = "src/day8_input.txt";
    // let filename = "src/day8_example.txt";
    let filename = "src/day8_example_3.txt";
    let file_content = fs::read_to_string(filename).expect("error");
    let file_lines: Vec<&str> = file_content.split("\n").filter(|&line| line != "").collect();
    let instructions: Vec<char> = file_lines[0].chars().collect();
    println!("instructions: {:?}", instructions);

    // part1(file_lines, instructions)

    // part3
    let label_to_node = build_label_to_nodes(file_lines);
    let nodes_vector: Vec<&Node> = label_to_node.values().clone().collect();

    let start_nodes: Vec<&Node> = nodes_vector
        .into_iter()
        .filter(|node| node.label.chars().nth(2) == Some('A'))
        .collect::<Vec<_>>();

    println!("Starting nodes {:?}", start_nodes);

    let node = start_nodes[0];
    let steps = 0;

    println!("First node {:?}", node);
    let visited_nodes: Vec<String> = vec![];
    let result = do_step_part2(&label_to_node, instructions, node, steps, visited_nodes);
    println!("Result 2: {:?}", result)
}
