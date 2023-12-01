use std::fmt::format;

fn main() {
    let input  = include_str!("./input1.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> i32 {
    let result = input.lines().map(|line| {
        let numbers: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        return format!("{}{}", first, last).parse::<i32>().unwrap();
    }).reduce(|a, b| a + b).unwrap();

    return result;
}

#[cfg(test)]
mod tests{
    use crate::part1;

    #[test]
    fn solution(){
        let result = part1("");
        assert_eq!(result, "")
    }
}