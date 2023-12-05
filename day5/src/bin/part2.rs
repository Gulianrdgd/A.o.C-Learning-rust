use std::collections::{HashMap};

fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}

struct ConversionVector {
    to: String,
    from_start: Vec<u64>,
    to_start: Vec<u64>,
    amount: Vec<u64>,
}


fn part2(input: &str) -> u64 {
    let seeds = get_seeds(input);
    let sections = get_sections(input);

    let mut conversion_map = HashMap::new();

    for sec in sections{
        let (from, to, mut from_start, mut to_start, mut amount) = parse_conversion_sections(sec);
        from_start.reverse();
        to_start.reverse();
        amount.reverse();
        conversion_map.insert(from, ConversionVector {
            to,
            from_start,
            to_start,
            amount,
        });
    }
    let mut result= 93402823665;

    let mut previous_range_run = Vec::new();


    println!("all ready to go!");
    for (index, s) in seeds.iter().enumerate(){
        println!("{} of {}", index, seeds.len());
        let (s_clone, amount_clone) = s.clone();

        for n in s_clone..s_clone+amount_clone+1 {
            let mut temp = n.clone();
            let mut to = "SEED";

            if is_in_previous_ranges(n, &previous_range_run) {
                println!("We found it before!");
                continue;
            }

            while to != "LOCATION" {
                let conv = conversion_map.get(to).unwrap();
                to = conv.to.trim();

                for conv_index in 0..conv.from_start.len() {
                    if temp >= conv.from_start[conv_index] && temp <= conv.from_start[conv_index] + conv.amount[conv_index] {
                        temp = conv.to_start[conv_index] + (temp - conv.from_start[conv_index]);
                        break;
                    }
                }
            }

            if result > temp {
                println!("{} is smaller than {}", temp, result);
                result = temp.clone();
            }
        }

        previous_range_run.push(s.clone());
    }

    return result;
}

fn is_in_previous_ranges(n:u64, previous_range_run: &Vec<(u64, u64)>) -> bool {
    for (start, pr_amount) in previous_range_run {
        if n>=*start && n<=*start+*pr_amount {
            return true;
        }
    }
    return false;
}

fn get_seeds(input: &str) -> Vec<(u64, u64)>{
    let mut result = Vec::new();
    let mut nums = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().trim().split(" ");

    let mut count = nums.clone().count();
    while count != 0 {
        let first = nums.nth(0).unwrap().parse::<u64>().unwrap();
        let second = nums.nth(0).unwrap().parse::<u64>().unwrap();

        result.push((first, second));
        count = count -2;
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

fn parse_conversion_sections(section: String) -> (String, String, Vec<u64> , Vec<u64>, Vec<u64>){
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

        let mut nums = sec.split(" ").map(|num| num.parse::<u64>().unwrap());
        to_start.push(nums.nth(0).unwrap());
        from_start.push(nums.nth(0).unwrap());
        amount.push(nums.nth(0).unwrap());
    }

    return (from.to_uppercase(), to.to_uppercase(), from_start, to_start, amount);
}
