use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}

struct ConversionVector {
    to: String,
    from_start: Vec<i64>,
    to_start: Vec<i64>,
    amount: Vec<i64>,
}


fn part2(input: &str) -> i64 {
    let seeds = get_seeds(input);
    let sections = get_sections(input);

    let mut seeds_clone = seeds.clone();
    let mut temp_seeds: Vec<(i64, i64)> =seeds_clone.clone();

    for sec in sections{
        println!("Sec: {}", sec);
        let (from, to, from_start, to_start, amount) = parse_conversion_sections(sec);

        seeds_clone = temp_seeds.clone();
        temp_seeds = Vec::new();

        for (seed, s_amount) in seeds_clone {
            for conv_index in 0..from_start.len() {
                // It fits perfectly
                if(seed > from_start[conv_index]  && seed + s_amount < from_start[conv_index] + amount[conv_index] ){
                    let first_val = to_start[conv_index] + (seed - from_start[conv_index]);
                    temp_seeds.push((first_val, s_amount));
                }else if(seed >= from_start[conv_index] &&  seed + s_amount > from_start[conv_index] + amount[conv_index]){
                    // the start is within the range but a part is not
                    let first_val = to_start[conv_index] + (seed - from_start[conv_index]);
                    let amount_out = (seed + s_amount) - from_start[conv_index] + amount[conv_index];
                    let amount_in = s_amount - amount_out;

                    temp_seeds.push((first_val, amount_in));
                    temp_seeds.push((seed+amount_in, amount_out));
                }else if(seed < from_start[conv_index] && seed + s_amount <= from_start[conv_index] + amount[conv_index]){
                    // The start is not in the range but a later part is
                    let amount_out = from_start[conv_index] - seed;
                    let amount_in = s_amount - amount_out;
                    temp_seeds.push((seed, amount_out));
                    temp_seeds.push((to_start[conv_index], amount_out));
                }else if(seed < from_start[conv_index] && seed + s_amount > from_start[conv_index] + amount[conv_index]){
                    let amount_first_out = from_start[conv_index] - seed;
                    let amount_last_out = s_amount - from_start[conv_index] + amount[conv_index];
                    temp_seeds.push((seed, amount_first_out));
                    temp_seeds.push((from_start[conv_index] + amount[conv_index], amount_last_out));
                    temp_seeds.push((to_start[conv_index], amount[conv_index]));
                }else{
                    temp_seeds.push((seed, s_amount));
                }
            }
        }
    }

    let result = temp_seeds.iter().cloned().map(|(x, _)| x).reduce(|a, b| min(a, b)).unwrap();


    return result;
}

fn get_seeds(input: &str) -> Vec<(i64, i64)>{
    let mut result = Vec::new();
    let mut nums = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().trim().split(" ");

    let mut count = nums.clone().count();
    while count != 0 {
        let mut first = nums.nth(0).unwrap().parse::<i64>().unwrap();
        let mut second = nums.nth(0).unwrap().parse::<i64>().unwrap();

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
