use std::collections::HashMap;

fn main() {
    let input  = include_str!("./input2.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> i64 {
    let instructions = input.lines().nth(0).unwrap().chars().map(|x| x).collect::<Vec<char>>();
    let mut map_instruction = "".to_owned();

    for (index, line) in input.lines().enumerate(){
        if index <= 1{
            continue;
        }

        map_instruction += &*line.replace(" ", "").replace("(", "").replace(")", "\n");
    }

    let (map, starting_pos) = parse_map(map_instruction);

    let mut loops = Vec::new();

    for pos in starting_pos{
        loops.push(traverse_map(pos, map.clone(), instructions.clone()));
    }

    // for l in loops {
    //     result = math
    // }
    let result= lcm(&loops);

    return result;

}


pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

fn traverse_map(start_node: String, map: HashMap<String, (String, String)>, instructions: Vec<char>) -> i64{
    let mut curr_node = start_node.as_str();
    let mut instruction_index = 0;
    let mut steps = 0;

    while !curr_node.ends_with("Z") {
            let (dir_l, dir_r)= map.get(curr_node).unwrap();

            if instructions[instruction_index] == 'L'{
                curr_node = dir_l;
            }else {
                curr_node = dir_r;
            }
        steps +=1;
        instruction_index += 1;
        if instruction_index == instructions.len(){
            instruction_index = 0;
        }
        }

    return steps;
}