fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> i32 {
    let result = input.lines().map(|line| get_min_power(line)).collect::<Vec<i32>>().into_iter().reduce(|a, b| a+b).unwrap();

    return result;
}

fn get_min_power(game: &str) -> i32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    let mut game_removed = game.split(":");

    for seperate_game in game_removed.nth(1).unwrap().split(";"){
        let rgb = seperate_game.split(",");
        for dice_throw in rgb{
            let dice = dice_throw.trim().split(" ").collect::<Vec<&str>>();
            let amount = dice[0].parse::<i32>().unwrap();
            let color = dice[1];

            if color == "red" && min_red < amount{
                min_red = amount;
            }else if color == "green" && min_green < amount{
                min_green = amount;
            }else if color == "blue" && min_blue < amount{
                min_blue = amount;
            }
        }
    }
    return min_red * min_green * min_blue;
}

#[cfg(test)]
mod tests{
    use crate::part2;

    #[test]
    fn solution(){
        let result = part2("");
        assert_eq!(result, "")
    }
}