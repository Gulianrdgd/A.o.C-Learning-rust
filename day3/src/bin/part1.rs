use std::ops::Deref;

fn main() {
    let input  = include_str!("./input1.txt");
    let (nums, parts) = part1(input);

    let result = nums.iter().filter(|num| {
        for p in &parts{
            if is_neighbour(num.deref(), p){
                return true;
            }
        }
        return false;
    }).map(|num| num.val).reduce(|a, b| a + b).unwrap();

    println!("{}", result)
}

struct Number {
    row_num: i32,
    start_index: i32,
    end_index: i32,
    val: i32
}

struct Part{
    row_num: i32,
    index: i32,
    val: char
}

fn part1(input: &str) -> (Vec<Number>, Vec<Part>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut number_temp = Number{
        row_num: 0,
        start_index: 0,
        end_index: 0,
        val: 0,
    };
    let mut found_number = false;


    for (row_num, row_val) in input.lines().enumerate(){
        let row_size = row_val.len();
        if found_number {
            found_number = false;
            numbers.push(Number {
                row_num: number_temp.row_num,
                start_index: number_temp.start_index,
                end_index: (row_size - 1) as i32,
                val: number_temp.val,
            });
        }
        for (col_num, col_val) in row_val.chars().enumerate(){
            if col_val.is_numeric() && !found_number{
                found_number = true;
                number_temp = Number{
                    row_num: row_num as i32,
                    start_index: col_num as i32,
                    end_index: 0,
                    val: col_val.to_digit(10).unwrap() as i32
                }
            }else if col_val.is_numeric() && found_number{
                number_temp = Number{
                    row_num: number_temp.row_num,
                    start_index: number_temp.start_index,
                    end_index: 0,
                    val: number_temp.val*10 + col_val.to_digit(10).unwrap() as i32
                }
            }else if !col_val.is_numeric() {
                if found_number {
                    found_number = false;
                    numbers.push(Number {
                        row_num: number_temp.row_num,
                        start_index: number_temp.start_index,
                        end_index: (col_num - 1) as i32,
                        val: number_temp.val,
                    });
                }
                if col_val != '.'{
                    parts.push(Part{
                        row_num: row_num as i32,
                        index: col_num as i32,
                        val: col_val,
                    });
                }
            }
        }
    }

    return (numbers, parts);
}

fn is_neighbour(num: &Number, part: &Part) -> bool{
    if (num.row_num == part.row_num
        && (part.index == num.start_index- 1 || part.index == num.end_index + 1)){
        // println!("Yes same row {} {}", part.val, num.val);
        return true;
    }

    if ((num.row_num + 1 == part.row_num) || (num.row_num - 1 == part.row_num)) && (part.index >= num.start_index -1 && part.index <= num.end_index + 1){
        // println!("Above or below {} part index:{}  {}, start: {} end: {}", part.val, part.index, num.val, num.start_index, num.end_index);
        return true;
    }
    return false;
}