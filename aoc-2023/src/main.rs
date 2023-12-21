use std::{fs, char};

fn main() {
    #[derive(Debug)]
    struct HandTypes {
        five: bool,
        four: bool,
        full: bool,
        three: bool,
        two_pair: bool,
        pair: bool,
    }
    fn new_hand_types() -> HandTypes {
        HandTypes {
            five: false,
            four: false,
            full: false,
            three: false,
            two_pair: false,
            pair: false,
        }
    }
    #[derive(Debug)]
    struct Hand {
        cards: String,
        bet: u32,
        hand_type: HandTypes,
        rank: u32,
    }
    fn read_file(file_path: &str) -> String {
        println!("In file {}", file_path); 
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        contents
    }
    fn return_hands(lines: String) -> Vec<Hand> {
        let mut all_hands: Vec<Hand> = Vec::new();
        for(index, line) in lines.lines().enumerate() {
            let v: Vec<&str> = line.split(' ').collect();
            let mut hand_types = new_hand_types();
            let mut hand: Hand = Hand { 
                cards: v[0].to_string(), 
                bet: v[1].parse::<u32>().unwrap(), 
                hand_type: hand_types,
                rank: 0,
            };
            all_hands.push(hand);
        }
        all_hands
    }
    fn find_type(hands: Vec<Hand>) -> Vec<Hand> {
        let c = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
        for hand in hands.iter() {
            println!("{}", hand.cards);
            for crd in c.clone() {
                println!("crd {}", crd);
                let v: Vec<_> = hand.cards.match_indices(crd).collect();
                // if hand.hand_type.pair == true {
                //     match v.len() {
                //         
                //     }

                // }
                match v.len() {
                    5 => hand.hand_type.five = true,
                    4 => hand.hand_type.four = true,
                    3 => hand.hand_type.three = true,
                    2 => hand.hand_type.pair = true,
                    _ => hand.hand_type.five = true,
                }

                // match count {
                //     Some(value) => println!("{:?}", value),
                //     None => {}
                // }
                println!("{:?}", v);
                println!("{:?}", v.len());

            }
        }
        hands
    }

    let lines = read_file("./day7/test_input");
    let hand_vec = return_hands(lines);
    find_type(hand_vec);
    // println!("{:?}", hand_vec);

}
