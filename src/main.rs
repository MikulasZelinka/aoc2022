use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn p01_1() {
    let mut current = 0;
    let mut max: i32 = 0;

    if let Ok(lines) = read_lines("assets/01.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    if current > max {
                        max = current;
                    }
                    current = 0;
                    continue;
                }
                let calories: i32 = line.parse().unwrap();
                current += calories;
            }
        }
        println!("{}", max);
    } else {
        println!("error reading file");
    }
}

fn p01_2() {
    let mut current = 0;

    let mut heap = BinaryHeap::new();
    const TOP_N: u32 = 3;

    if let Ok(lines) = read_lines("assets/01.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    heap.push(current);
                    current = 0;
                    continue;
                }
                let calories: i32 = line.parse().unwrap();
                current += calories;
            }
        }

        let mut total: i32 = 0;
        for _ in 1..=TOP_N {
            total += heap.pop().unwrap();
        }

        println!("{}", total);
    } else {
        println!("error reading file");
    }
}

fn p02(part_two: bool) {
    #[derive(PartialEq, EnumIter)]
    enum Hand {
        Rock,
        Paper,
        Scissors,
    }

    fn value(hand: &Hand) -> u32 {
        match hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn win(hand: &Hand) -> Hand {
        for response in Hand::iter() {
            if &response > hand {
                return response;
            }
        }
        // very ugly:
        return Hand::Rock;
    }

    fn draw(hand: &Hand) -> Hand {
        for response in Hand::iter() {
            if &response == hand {
                return response;
            }
        }
        // very ugly:
        return Hand::Rock;
    }

    fn lose(hand: &Hand) -> Hand {
        for response in Hand::iter() {
            if &response < hand {
                return response;
            }
        }
        // very ugly:
        return Hand::Rock;
    }

    impl FromStr for Hand {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Hand::Rock),
                "B" => Ok(Hand::Paper),
                "C" => Ok(Hand::Scissors),
                "X" => Ok(Hand::Rock),
                "Y" => Ok(Hand::Paper),
                "Z" => Ok(Hand::Scissors),
                _ => Err(()),
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self {
                Hand::Rock => match other {
                    Hand::Rock => 0.partial_cmp(&0),
                    Hand::Paper => 0.partial_cmp(&1),
                    Hand::Scissors => 1.partial_cmp(&0),
                },
                Hand::Paper => match other {
                    Hand::Rock => 1.partial_cmp(&0),
                    Hand::Paper => 0.partial_cmp(&0),
                    Hand::Scissors => 0.partial_cmp(&1),
                },
                Hand::Scissors => match other {
                    Hand::Rock => 0.partial_cmp(&1),
                    Hand::Paper => 1.partial_cmp(&0),
                    Hand::Scissors => 0.partial_cmp(&0),
                },
            }
        }
    }

    fn get_score(line: String, part_two: bool) -> u32 {
        let hand_opponent = Hand::from_str(&line[0..1]).unwrap();
        let mut hand_player = Hand::from_str(&line[2..3]).unwrap();

        if part_two {
            hand_player = match hand_player {
                Hand::Rock => lose(&hand_opponent),
                Hand::Paper => draw(&hand_opponent),
                Hand::Scissors => win(&hand_opponent),
            };
        }

        value(&hand_player)
            + if hand_player > hand_opponent {
                6
            } else if hand_player == hand_opponent {
                3
            } else {
                0
            }
    }

    let mut score = 0;

    if let Ok(lines) = read_lines("assets/02.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }
                score += get_score(line, part_two);
            }
        }

        println!("{}", score);
    } else {
        println!("error reading file");
    }
}

fn p03(part_two: bool) {
    let mut priorities: u32 = 0;
    let mut options: HashSet<u8> = HashSet::new();

    fn get_priority(c: u8) -> u32 {
        let mut priority: u8 = 1;
        if b'a' <= c {
            priority += c - b'a';
        } else {
            priority += c - b'A' + 26;
        }
        priority as u32
    }

    if let Ok(lines) = read_lines("assets/03.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }

                if part_two {
                    let new_candidates: HashSet<u8> = HashSet::from_iter(line.bytes());

                    if options.is_empty() {
                        options = new_candidates;
                    } else {
                        options.retain(|x| new_candidates.contains(x));
                    }

                    if options.len() == 1 {
                        priorities += get_priority(*options.iter().next().unwrap());
                        options.clear();
                    }
                } else {
                    let (left, right) = line.split_at(line.len() / 2);
                    let left: HashSet<u8> = HashSet::from_iter(left.bytes());

                    for c in right.bytes() {
                        if left.contains(&c) {
                            priorities += get_priority(c);
                            break;
                        }
                    }
                }
            }
        }
        println!("{}", priorities);
    } else {
        println!("error reading file");
    }
}

fn main() {
    println!("Hello, advent!");

    p03(true);
    p03(false);
    p02(true);
    p02(false);
    p01_2();
    p01_1();
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// (ugh, why is there no standard function for this?)
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
