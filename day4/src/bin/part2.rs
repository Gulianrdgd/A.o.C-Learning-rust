fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}


fn part2(input: &str) -> i32 {
    let matches_amount = input.lines().map(|card| {
        let nums = card.split(":").collect::<Vec<&str>>().last().unwrap().split("|").collect::<Vec<&str>>();

        let winning_nums = nums.first().unwrap().trim().replace("  ", " ").split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let our_nums = nums.last().unwrap().trim().replace("  ", " ").split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let matches = our_nums.iter().filter(|n| winning_nums.contains(n)).collect::<Vec<&i32>>();

        return matches.len();
    }).collect::<Vec<usize>>();

    let mut occurances = vec![1; matches_amount.len()];

    for (index , match_a) in matches_amount.iter().enumerate() {
        for n in index+1..index+match_a+1{
            // println!("{} m_a {} {}..{}", n, match_a, index, index+match_a);
            occurances[n] = occurances[n] + occurances[index];
        }
    }

    let result = occurances.iter().cloned().reduce(|a, b| a + b).unwrap();

    return result.clone();
}
