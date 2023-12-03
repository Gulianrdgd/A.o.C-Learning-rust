fn main() {
    let input  = include_str!("./input1.txt");
    let (nums, parts) = part1(input);
    let first_res = nums.first().unwrap();
    let first_part = parts.first().unwrap();
    println!("{} {} {} {} ", first_res.val, first_res.start_index, first_res.row_num, first_res.end_index);
    println!("{} {} {}", first_part.row_num, first_part.val, first_part.index)
}

struct Number {
    row_num: usize,
    start_index: usize,
    end_index: usize,
    val: u32
}

struct Part{
    row_num: usize,
    index: usize,
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
                end_index: row_size - 1,
                val: number_temp.val,
            });
        }
        for (col_num, col_val) in row_val.chars().enumerate(){
            if col_val.is_numeric() && !found_number{
                println!("{}", col_val);
                found_number = true;
                number_temp = Number{
                    row_num,
                    start_index: col_num,
                    end_index: 0,
                    val: col_val.to_digit(10).unwrap()
                }
            }else if col_val.is_numeric() && found_number{
                number_temp = Number{
                    row_num: number_temp.row_num,
                    start_index: number_temp.start_index,
                    end_index: 0,
                    val: number_temp.val*10 + col_val.to_digit(10).unwrap()
                }
            }else if !col_val.is_numeric() {
                if found_number {
                    found_number = false;
                    numbers.push(Number {
                        row_num: number_temp.row_num,
                        start_index: number_temp.start_index,
                        end_index: col_num -1,
                        val: number_temp.val,
                    });
                }
                if col_val != '.'{
                    parts.push(Part{
                        row_num,
                        index: col_num,
                        val: col_val,
                    });
                }
            }
        }
    }

    return (numbers, parts);
}