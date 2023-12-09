fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}

fn part2(input: &str) -> i64 {
    let mut result = 1;
    for inp in input.lines(){
        let triangle = create_triangle(inp.split(" ").map(|x| x.parse::<i64>().unwrap()).collect());
        let mut val = 0;
        for row in triangle.iter().rev(){
            val = row[0] - val;
        }
        result += val;
    }

    return result;
}

fn create_triangle(input: Vec<i64>) -> Vec<Vec<i64>>{
    let mut triangle = Vec::new();

    triangle.push(input);
    let mut all_zero = false;
    let mut row_index = 0;

    while !all_zero{
        let mut row = Vec::new();

        for index in 0..triangle[row_index].len() - 1{
            let val =  triangle[row_index][index + 1] - triangle[row_index][index];

            if val == 0 {
                all_zero = true;
            }else{
                all_zero = false;
            }

            row.push(val);
        }

        triangle.push(row);
        row_index += 1;
    }

    return triangle;
}