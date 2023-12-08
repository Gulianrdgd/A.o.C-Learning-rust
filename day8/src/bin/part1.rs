use std::collections::HashMap;

fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> i32 {
    let instructions = input.lines().nth(0).unwrap().chars().map(|x| x).collect::<Vec<char>>();
    let mut map_instruction = "".to_owned();

    for (index, line) in input.lines().enumerate(){
        if index <= 1{
            continue;
        }

        map_instruction += &*line.replace(" ", "").replace("(", "").replace(")", "\n");
    }

    let map = parse_map(map_instruction);

    let mut curr_node = "AAA";
    let mut instruction_index = 0;
    let mut steps = 0;

    while curr_node != "ZZZ" {
        let (dir_l, dir_r)= map.get(curr_node).unwrap();

        if instructions[instruction_index] == 'L'{
            curr_node = dir_l.trim();
        }else {
            curr_node = dir_r.trim();
        }

        steps +=1;
        instruction_index += 1;
        if instruction_index == instructions.len(){
            instruction_index = 0;
        }
    }

    return steps;
}

fn parse_map(input: String) -> HashMap<String, (String, String)> {

    let mut map = HashMap::new();


    for line in input.lines() {
        let mut split = line.split("=");

        let node = split.nth(0).unwrap();

        let mut lr = split.nth(0).unwrap().split(",");
        map.insert(node.to_owned(), (lr.nth(0).unwrap().to_owned(), lr.nth(0).unwrap().to_owned()));
    }

    return map;
}