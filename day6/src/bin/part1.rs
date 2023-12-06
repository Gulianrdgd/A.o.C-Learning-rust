fn main() {
    let input  = include_str!("./input1.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> i64 {
    let times = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let distance = input.lines().nth(1).unwrap().split(":").nth(1).unwrap().split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut result = 1;

    for i in 0..times.len(){
        let mut press_time = 0;
        let mut wins = 0;

        while press_time <= times[i]{
            let reached_dist = (times[i] - press_time) * press_time;
            if reached_dist > distance[i]{
                wins +=1;
            }
            press_time += 1;
        }
        println!("wins: {}", wins);
        result *= wins;
    }
    return result;
}