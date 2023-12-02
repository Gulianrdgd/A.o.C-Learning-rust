fn main() {
    let input  = include_str!("./input1.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> i32 {
    let filtered_games: Vec<&str> = input.lines().filter(|line| is_possibe_game(line)).collect();
    let result = filtered_games.iter().map(|game| {
        let game_number = game.split(" ").nth(1).unwrap().replace(":", "").parse::<i32>().unwrap();

        return game_number;
    }).reduce(|a, b | a + b).unwrap();
    return result;
}

fn is_possibe_game(game: &str) -> bool {
    let red = 12;
    let green = 13;
    let blue = 14;

    let mut game_removed = game.split(":");

    for seperate_game in game_removed.nth(1).unwrap().split(";"){
        let rgb = seperate_game.split(",");
        for dice_throw in rgb{
            let mut dice = dice_throw.trim().split(" ").collect::<Vec<&str>>();
            let amount = dice[0].parse::<i32>().unwrap();
            let color = dice[1];

            if (color == "red" && amount > red) || (color == "green" && amount > green) || (color == "blue" && amount > blue){
                return false;
            }
        }

    }
    return true;
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