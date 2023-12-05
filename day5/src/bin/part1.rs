use std::collections::HashMap;

fn main() {
    let input  = include_str!("./input1.txt");
    let result = part1(input);
    println!("{}", result);
}

struct ConversionVector {
    to: String,
    from_start: Vec<i64>,
    to_start: Vec<i64>,
    amount: Vec<i64>,
}


fn part1(input: &str) -> i64 {
    let seeds = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().trim().split(" ").map(|seed| seed.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let sections = get_sections(input);

    let mut conversion_map = HashMap::new();

    for sec in sections{
        let (from, to, from_start, to_start, amount) = parse_conversion_sections(sec);
        conversion_map.insert(from, ConversionVector {
            to,
            from_start,
            to_start,
            amount,
        });
    }
    let mut result= 0;


    for (index, s) in seeds.iter().enumerate(){
        let mut temp = s.clone();
        let mut to = "SEED";

        while to != "LOCATION" {
            let conv = conversion_map.get(to).unwrap();
            to = conv.to.trim();

            for conv_index in 0..conv.from_start.len() {
                if temp >= conv.from_start[conv_index] && temp <= conv.from_start[conv_index] + conv.amount[conv_index]{
                    temp = conv.to_start[conv_index] + (temp - conv.from_start[conv_index]);
                    break;
                }
            }

        }

        if index == 0 || result > temp {
            result = temp;
        }
    }

    return result;
}

fn get_sections(input: &str) -> Vec<String> {
    let mut ret = Vec::new();
    let mut temp = "".to_uppercase();

    for (index, line) in input.lines().enumerate(){
        if index <= 1 {
            continue;
        }

        if line == ""{
            ret.push(temp);
            temp = "".to_uppercase();
            continue;
        }

        temp = format!("{}{}\n", temp, line);
    }

    ret.push(temp);

    return ret;
}

fn parse_conversion_sections(section: String) -> (String, String, Vec<i64> , Vec<i64>, Vec<i64>){
    let mut from = "";
    let mut to = "";

    let mut amount = Vec::new();
    let mut to_start = Vec::new();
    let mut from_start = Vec::new();

    for (index, sec) in section.lines().enumerate(){
        if index == 0 {
            let mut from_to = sec.split(" ").nth(0).unwrap().split("-");

            from = from_to.nth(0).unwrap().trim();
            to = from_to.nth(1).unwrap().trim();

            continue;
        }

        let mut nums = sec.split(" ").map(|num| num.parse::<i64>().unwrap());
        to_start.push(nums.nth(0).unwrap());
        from_start.push(nums.nth(0).unwrap());
        amount.push(nums.nth(0).unwrap());
    }

    return (from.to_uppercase(), to.to_uppercase(), from_start, to_start, amount);
}
