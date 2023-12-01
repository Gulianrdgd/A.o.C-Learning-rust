fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> i32 {
    let replaced_string: Vec<String> = input.lines().map(|line| {
        let mut temp_string: String = "".to_string();
        let mut first = "".to_string();
        let mut last = "".to_string();

            for l_char in line.chars(){
                temp_string = format!("{}{}", temp_string, l_char);

                if l_char.is_numeric(){
                    first = l_char.to_string();
                    break;
                }

                let index = find_number_in_string(temp_string.clone());
                if index != -1{
                    first = index.to_string();
                    break;
                }
            }

        temp_string = "".to_string();

        for l_char in line.chars().rev(){
            temp_string = format!("{}{}", l_char,  temp_string);

            if l_char.is_numeric(){
                last = l_char.to_string();
                break;
            }

            let index = find_number_in_string(temp_string.clone());
            if index != -1{
                last = index.to_string();
                break;
            }
        }

        return format!("{}{}", first, last);
    }).collect();
    //
    let result = replaced_string.iter().map(|line| line.parse::<i32>().unwrap()).reduce(|a, b| a + b).unwrap();

    return result;
}


fn find_number_in_string(temp_string: String) -> i32{
    let possible_numbers: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for (i, p_num) in possible_numbers.iter().enumerate() {
        if temp_string.contains(p_num){
            return (i + 1) as i32;
        }
    }
    return -1;
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