use std::collections::HashMap;

fn main() {
    let input  = include_str!("./input2.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> i32 {
    let instructions = input.lines().nth(0).unwrap().chars().map(|x| x).collect::<Vec<char>>();
    let mut map_instruction = "".to_owned();

    for (index, line) in input.lines().enumerate(){
        if index <= 1{
            continue;
        }

        map_instruction += &*line.replace(" ", "").replace("(", "").replace(")", "\n");
    }

    let (map, starting_pos) = parse_map(map_instruction);


    return traverse_map(starting_pos, map, instructions);

}

fn parse_map(input: String) -> (HashMap<String, (String, String)>, Vec<String>) {
    let mut map = HashMap::new();
    let mut starting_positions = Vec::new();

    for line in input.lines() {
        let mut split = line.split("=");

        let node = split.nth(0).unwrap();

        if node.ends_with("A"){
            starting_positions.push(node.to_owned());
        }

        let mut lr = split.nth(0).unwrap().split(",");
        map.insert(node.to_owned(), (lr.nth(0).unwrap().to_owned(), lr.nth(0).unwrap().to_owned()));
    }

    return (map, starting_positions);
}

fn traverse_map(start_node: Vec<String>, map: HashMap<String, (String, String)>, instructions: Vec<char>) -> i32{
    let mut curr_node = start_node;
    let mut instruction_index = 0;
    let mut steps = 0;

    while curr_node.iter().filter(|x| !x.ends_with("Z")).count() != 0 {
        for (index, curr) in curr_node.clone().iter().enumerate() {
            let (dir_l, dir_r)= map.get(curr).unwrap();

            if instructions[instruction_index] == 'L'{
                curr_node[index] = dir_l.clone();
            }else {
                curr_node[index] = dir_r.clone();
            }
        }
        steps +=1;
        instruction_index += 1;
        if instruction_index == instructions.len(){
            instruction_index = 0;
        }
    }
    return steps;
}