fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> i64 {
    let times = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().replace(" ", "").parse::<i64>().unwrap();
    let distance = input.lines().nth(1).unwrap().split(":").nth(1).unwrap().replace(" ", "").parse::<i64>().unwrap();

    let mut result = 1;

    let mut press_time = 0;
    let mut wins = 0;

    while press_time <= times{
        let reached_dist = (times - press_time) * press_time;
        if reached_dist > distance{
            wins +=1;
        }

        press_time += 1;
    }

    result *= wins;

    return result;
}