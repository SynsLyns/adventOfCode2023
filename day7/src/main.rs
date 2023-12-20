use std::collections::HashMap;
use std::fs;
use std::env;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u64,
    hand_type: HandType
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl Hand {
    fn import_from_lines(lines: Vec<&str>) -> Vec<Vec<Hand>> {
        let mut ordered_cards: Vec<Vec<Hand>> = vec![];
        for _ in 0..7 {
            ordered_cards.push(Vec::new());
        }
        for line in lines {
            let hand = Hand::import_from_line(line);
            match hand.hand_type {
                HandType::FiveOfAKind => ordered_cards[6].push(hand),
                HandType::FourOfAKind => ordered_cards[5].push(hand),
                HandType::FullHouse => ordered_cards[4].push(hand),
                HandType::ThreeOfAKind => ordered_cards[3].push(hand),
                HandType::TwoPair => ordered_cards[2].push(hand),
                HandType::OnePair => ordered_cards[1].push(hand),
                HandType::HighCard => ordered_cards[0].push(hand),
            }
        }
        for vector in &mut ordered_cards {
            vector.sort_by(|a, b| b.cards.partial_cmp(&a.cards).unwrap());
            vector.reverse();
        }
        ordered_cards
    }

    fn import_from_line(line: &str) -> Hand {
        let mut data = line.split_whitespace();
        let mut map: HashMap<char, u8> = HashMap::new();
        let cards: String = data.next().unwrap().to_string().chars()
                        .map(|x| match x {
                            'J' => 'A',
                            '2' => 'B',
                            '3' => 'C',
                            '4' => 'D',
                            '5' => 'E',
                            '6' => 'F',
                            '7' => 'G',
                            '8' => 'H',
                            '9' => 'I',
                            'T' => 'J',
                            'Q' => 'K',
                            'K' => 'L',
                            'A' => 'M',
                             _ => '0'
                        }).collect();
        for card in cards.chars() {
            map.entry(card).and_modify(|counter| *counter += 1).or_insert(1);
        }

        let wildcards = match map.remove(&'A') {
            Some(value) => value,
            None => 0
        };
        match map.iter().max_by_key(|entry| entry.1) {
            Some((key_with_max_value, _)) => {
                map.entry(*key_with_max_value).and_modify(|x| *x += wildcards);
            }
            None => {
                map.insert('M', 5);
            }
        };

        let hand_type: HandType = match map.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let (_, value) = map.iter().max_by_key(|&(_, value)| value).unwrap();
                match value {
                    4 => HandType::FourOfAKind,
                    3 => HandType::FullHouse,
                    _ => HandType::HighCard
                }
            }
            3 => {
                let (_, value) = map.iter().max_by_key(|&(_, value)| value).unwrap();
                match value {
                    3 => HandType::ThreeOfAKind,
                    2 => HandType::TwoPair,
                    _ => HandType::HighCard
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard
        };

        Hand {
            cards,
            bid: data.next().unwrap().parse().unwrap(),
            hand_type
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Opening file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let hands = Hand::import_from_lines(contents.lines().collect());
    let mut part1 = 0;
    let mut rank = 1;
    for hand_type in hands {
        for hand in hand_type {
            part1 += hand.bid * rank;
            rank += 1;
        }
    }
    println!("Part 1: {:?}", part1);
}
