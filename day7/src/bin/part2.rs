use std::cmp::Ordering;

fn main() {
    let input  = include_str!("./input2.txt");
    let result = part2(input);
    println!("{}", result);
}

#[derive(Ord, Eq)]
struct ExtendedHand {
    hand: String,
    hand_power: usize,
    hand_number: i32,
}

impl PartialEq<Self> for ExtendedHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_power == other.hand_power && self.hand == other.hand
    }
}

impl PartialOrd for ExtendedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_power == other.hand_power {
            return Some(cmp_hands(&self.hand, &other.hand));
        }
        Some(self.hand_power.cmp(&other.hand_power))
    }
}


fn part2(input: &str) -> i32 {
    let hands = input.lines().map(|a| {
        let mut splitted = a.split(" ").clone();
        return (splitted.nth(0).unwrap(), splitted.nth(0).unwrap().parse::<i32>().unwrap())
    }).collect::<Vec<(&str,i32)>>();

    let mut sorted_hands = hands.iter().map(|(hand, num)| {
        let oc_num = get_number_of_ocurrances(hand);
        let hand_power = decide_hand(oc_num);

        return ExtendedHand{
            hand: hand.to_string(),
            hand_power,
            hand_number: num.clone(),
        }
    }
    ).collect::<Vec<ExtendedHand>>();

    sorted_hands.sort();

    for i in 0..sorted_hands.len(){
        // if sorted_hands[i].hand.contains("J") {
            println!("{}: {} pow: {}", i, sorted_hands[i].hand, sorted_hands[i].hand_power);
        // }
    }

    let result = sorted_hands.iter().enumerate().map(|(index, ex_hand)| ((index as i32 ) +1 )*ex_hand.hand_number).reduce(|a, b| a+ b).unwrap();


    return result;
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
        if oc[0].val == 'J' || oc[1].val == 'J'{
            // Five of kind
            return 6;
        }

        return if (oc[0].amount == 2 && oc[1].amount == 3) || (oc[0].amount == 3 && oc[1].amount == 2) {
            // Full house
            4
        } else {
            // Four of a kind
            5
        }
    } else if same_cards == 3{
        if oc[0].val == 'J' || oc[1].val == 'J' || oc[2].val == 'J' {
            for o in oc.iter() {
                if o.amount == 3 {
                    return 5;
                }
                if o.val == 'J' {
                    if o.amount == 2 {
                        return 5;
                    }
                }
            }
            return 4;
        }

        // if oc[0].val == 'J' || oc[1].val == 'J' || oc[2].val == 'J'{
        //
        //     for o in oc.iter(){
        //         if o.amount == 3{
        //             return 5;
        //         }
        //     }
        //
        //     return 4;
        // }


            if (oc[0].amount == 3 && oc[1].amount == 1) ||
            (oc[1].amount == 3 && oc[0].amount == 1) ||
            (oc[2].amount == 3 && oc[0].amount == 1) {

            // Three of a kind
            return 3;
        } else {
            // Two pair
            return 2;
        }
    }else if same_cards == 4{
        for o in oc.iter(){
            if o.val == 'J'{
                return 3;
            }
        }
        // One pair
        return 1;
    }else if same_cards == 5 {
        for o in oc.iter(){
            if o.val == 'J'{
                return 1;
            }
        }

        // High card
        return 0
    }

    return 0;
}

fn transform_to_number(ch: char) -> i32 {
    if ch == 'T' {
        return 10;
    }

    if ch == 'J' {
        return 1;
    }

    if ch == 'Q'{
        return 12;
    }

    if ch == 'K'{
        return 13;
    }

    if ch == 'A'{
        return 14
    }

    return ch.to_digit(10).unwrap() as i32;
}

fn cmp_hands(hand1: &str, hand2: &str) -> Ordering{
    if hand1 == hand2{
        return Ordering::Equal;
    }

    let h1_c = hand1.chars().map(|a| a).collect::<Vec<char>>();
    let h2_c = hand2.chars().map(|a| a).collect::<Vec<char>>();

    for i in 0..h1_c.len(){
        let h1_conv = transform_to_number(h1_c[i]);
        let h2_conv = transform_to_number(h2_c[i]);
        if h1_conv == h2_conv {
            continue;
        }else if h1_conv > h2_conv {
            return Ordering::Greater;
        }else{
            return Ordering::Less;
        }
    }

    return Ordering::Less;
}