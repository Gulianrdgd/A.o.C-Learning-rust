use std::ops::Deref;

fn main() {
    let input  = include_str!("./input1.txt");
    let result = part1(input);
    println!("{}", result);
}


fn part1(input: &str) -> i32 {
    input.lines().map(|card| {
        let mut nums = card.split(":").collect::<Vec<&str>>().last().unwrap().split("|").collect::<Vec<&str>>();

        let winning_nums = nums.first().unwrap().trim().replace("  ", " ").split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let our_nums = nums.last().unwrap().trim().replace("  ", " ").split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let matches = our_nums.iter().filter(|n| winning_nums.contains(n)).collect::<Vec<&i32>>();

        if matches.len() > 0 {
            return 2_i32.pow((matches.len() - 1) as u32);
        }

        return 0;
    }).reduce(|a, b| a + b).unwrap() as i32
}
