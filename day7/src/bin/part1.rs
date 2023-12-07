fn main() {
    let input  = include_str!("./input1.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> i32 {
    let hands = input.clone().lines().map(|a| {
        let mut splitted = a.split(" ").clone();
        return (splitted.nth(0).unwrap(), splitted.nth(0).unwrap().parse::<i32>().unwrap())
    }).collect::<Vec<(&str,i32)>>();

    let result = hands.iter().map(|(hand, num)| {
        let oc_num = get_number_of_ocurrances(hand);
        let hand_power = decide_hand(oc_num);

        return (hand, (hand_power, num));
    }
    ).collect::<Vec<(&str, (usize, i32))>>().sort_by(|(a, (b, c)), (e, (f, g))|  b.partial_cmp(f).unwrap());
    // for (hand, num) in hands{
    //     let mut occurances = get_number_of_ocurrances(hand);
    //
    // }


    return result.unwrap();
}

struct Occurrence {
    val: char,
    amount: i32
}

fn get_number_of_ocurrances(input: &str) -> Vec<Occurrence>{
    let mut result = Vec::new();

    for c in input.chars(){
        let found_index =  result.iter().position(|x: &Occurrence| x.val == c);

        if found_index.is_some(){
            result[found_index.unwrap()].amount += 1;
        }else{
            result.push(Occurrence{
                val: c,
                amount: 1,
            })
        }
    }

    return result;
}

fn decide_hand(oc: Vec<Occurrence>) -> usize {
    let same_cards = oc.len();
    if same_cards == 1{
        // Five of a kind
        return 6;
    }else if same_cards == 2{
        return if (oc[0].amount == 2 && oc[1].amount == 3) || (oc[0].amount == 3 && oc[1].amount == 2) {
            // Full house
            4
        } else {
            // Four of a kind
            5
        }
    } else if same_cards == 3{
        return if ((oc[0].amount == 3 && oc[1].amount == 1) ||
            (oc[1].amount == 3 && oc[0].amount == 1) ||
            (oc[2].amount == 3 && oc[0].amount == 1)) {
            // Three of a kind
            3
        } else {
            // Two pair
            2
        }
    }else if same_cards == 4{
        // One pair
        return 1;
    }else if same_cards == 5 {
        // High card
        return 0
    }

    return 0;
}