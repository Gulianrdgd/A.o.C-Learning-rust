fn main() {
    let input  = include_str!("./input1.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> i32 {
    let (grid, (start_x, start_y)) = create_grid(input);
    let mut distances: Vec<Vec<i32>> = Vec::new();

    let mut loop_found = false;

    for row in &grid {
        let mut r = Vec::new();
        for c in row {
            r.push(-1);
        }
        distances.push(r);
    }

    distances[start_y as usize][start_x as usize] = 0;

    println!("{:?}", grid);
    println!("{:?}", distances);
    println!("{} {}", start_x, start_y);


    let mut left_to_check = Vec::new();

    let mut x = start_x;
    let mut y = start_y;


    while !loop_found{
        let steps = distances[y as usize][x as usize] + 1;
        // Check top
        if y != 0 {
            let (is_valid, (next_y, next_x)) = next_step(Direction::Down, grid[(y-1) as usize][x as usize]);
            if is_valid {
                distances[(y - 1) as usize][x as usize] = steps;

                println!("char:{} {} {} -> {} {}",grid[(y-1) as usize][x as usize], y, x, next_y, next_x);
                if distances[(y-1) as usize][x as usize] != -1 {
                    // println!("From: {} {}", x, y);
                    // println!("{:?}", distances);
                    let (other_x, other_y) = next_step(Direction::Up, grid[y as usize][x as usize]).1;
                    if distances[(y + other_y) as usize][(x + other_x) as usize] != -1 {
                        println!("{} {} s: {}", x, y, steps);
                        loop_found = true;
                        break;
                    }
                }else {
                    left_to_check.push((x + next_x, y + next_y - 1));
                }
            }

        }
        // Check bottom
        if y != (grid.len() - 1) as i32 {
            let (is_valid, (next_y, next_x)) = next_step(Direction::Up, grid[(y + 1) as usize][x as usize]);
            if is_valid {
                distances[(y + 1) as usize][x as usize] = steps;

                println!("char:{} {} {} -> {} {}", grid[(y + 1) as usize][x as usize], y, x, next_y, next_x);
                if distances[(y + 1) as usize][x as usize] != -1 {
                    // println!("From: {} {}", x, y);
                    // println!("{:?}", distances);
                    let (other_x, other_y) = next_step(Direction::Down, grid[y as usize][x as usize]).1;
                    if distances[(y + other_y) as usize ][(x + other_x) as usize] != -1 {
                        println!("{} {} s: {}", x, y, steps);
                        loop_found = true;
                        break;
                    }
                }else{
                    left_to_check.push((x + next_x, y + next_y + 1));
                }
            }
        }

        // Check left
        if x != 0 {
            let (is_valid, (next_y, next_x)) = next_step(Direction::Right, grid[y as usize][(x - 1) as usize]);
            if is_valid {
                distances[y as usize][(x - 1) as usize] = steps;

                println!("char:{} {} {} -> {} {}", grid[y as usize][(x - 1) as usize], y, x, next_y, next_x);
                if distances[y as usize][(x - 1) as usize] != -1 {
                    println!("From: {} {}", x, y);
                    println!("{:?}", distances);
                    let (other_x, other_y) = next_step(Direction::Left, grid[y as usize][x as usize]).1;
                    if distances[(y + other_y) as usize][(x + other_x) as usize] != -1 {
                        println!("{} {} s: {}", x, y, steps);
                        loop_found = true;
                        break;
                    }
                }else {
                    left_to_check.push((x + next_x - 1, y + next_y));
                }
            }
        }

        // Check right
        if x != grid[y as usize].len() as i32 - 1 {
            let (is_valid, (next_y, next_x)) = next_step(Direction::Left, grid[y as usize][(x + 1) as usize]);
            if is_valid {
                distances[y as usize][(x + 1) as usize] = steps;
                println!("char: {} {} {} -> {} {}",grid[y as usize][(x + 1) as usize], y, x, next_y, next_x);
                if distances[y as usize][(x + 1) as usize] != -1 {

                    let (other_x, other_y) = next_step(Direction::Right, grid[y as usize][x as usize]).1;
                    if distances[(y + other_y) as usize][(x + other_x) as usize] != -1 {
                        // println!("{} {} s: {}", x, y, steps);
                        loop_found = true;
                        break;
                    }
                }else {
                    left_to_check.push((x + next_x + 1, y + next_y));
                }
            }
        }

        if left_to_check.len() == 0 {
            break;
        }
        println!("Picking new start {:?}", left_to_check);
        let next = left_to_check.remove(0);
        x = next.0;
        y = next.1;
    }

    for d in &distances {
        println!("{:?}", d);
    }


    return 0;
}

fn create_grid(input: &str) -> (Vec<Vec<char>>, (i32, i32)){
    let mut grid = Vec::new();
    let mut s_x = 0;
    let mut s_y = 0;
    let mut found = false;

    for line in input.lines() {
        let mut row = Vec::new();
        if !found{
            s_x = 0;
        }

        for c in line.chars() {
            if c == 'S' {
                found = true;
            }

            if !found{
                s_x += 1;
            }

            row.push(c);
        }
        if !found{
            s_y += 1;
        }
        grid.push(row);
    }
    return (grid, (s_x, s_y));
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn next_step(comming_in_from: Direction, c: char) -> (bool, (i32, i32)){
    match c {
        '|' => {
            if comming_in_from == Direction::Up {
                return (true, (1, 0));
            } else if comming_in_from == Direction::Down {
                return (true, (-1, 0));
            } else {
                return (false, (0, 0));
            }
        },
        '-' => {
            if comming_in_from == Direction::Left {
                return (true, (0, 1));
            } else if comming_in_from == Direction::Right {
                return (true, (0, -1));
            } else {
                return (false, (0, 0));
            }
        },
        'L' => {
            if comming_in_from == Direction::Up {
                return (true, (0, 1));
            } else if comming_in_from == Direction::Right {
                return (true, (-1, 0));
            } else {
                return (false, (0, 0));
            }
        },
        'J' => {
            if comming_in_from == Direction::Up {
                return (true, (0, -1));
            } else if comming_in_from == Direction::Left {
                return (true, (-1, 0));
            } else {
                return (false, (0, 0));
            }
        },
        '7' => {
            if comming_in_from == Direction::Down {
                return (true, (0, -1));
            } else if comming_in_from == Direction::Left {
                return (true, (1, 0));
            } else {
                return (false, (0, 0));
            }
        },
        'F' => {
            if comming_in_from == Direction::Down {
                return (true, (0, 1));
            } else if comming_in_from == Direction::Right {
                return (true, (1, 0));
            } else {
                return (false, (0, 0));
            }
        },
        _ =>  (false, (0, 0))

    }
}