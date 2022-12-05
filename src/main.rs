use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
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

fn p04(part_two: bool) {
    fn split(x: &str, c: char) -> (&str, &str) {
        let mut split = x.split(c);
        (split.next().unwrap(), split.next().unwrap())
    }

    fn parse(x: &str) -> u32 {
        x.parse::<u32>().unwrap()
    }

    fn split_parse(x: &str, c: char) -> (u32, u32) {
        let (x1, x2) = split(x, c);
        (parse(x1), parse(x2))
    }
    let mut contained: u32 = 0;

    if let Ok(lines) = read_lines("assets/04.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }
                // println!("{line}");

                let (left, right) = split(&line, ',');

                let (x1, x2) = split_parse(left, '-');
                let (y1, y2) = split_parse(right, '-');

                if part_two {
                    if !(x2 < y1 || y2 < x1) {
                        contained += 1;
                    }
                } else {
                    if (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2) {
                        contained += 1;
                    }
                }
            }
        }
        println!("{}", contained);
    } else {
        println!("error reading file");
    }
}

fn p05(part_two: bool) {
    //             [J]             [B] [W]
    //             [T]     [W] [F] [R] [Z]
    //         [Q] [M]     [J] [R] [W] [H]
    //     [F] [L] [P]     [R] [N] [Z] [G]
    // [F] [M] [S] [Q]     [M] [P] [S] [C]
    // [L] [V] [R] [V] [W] [P] [C] [P] [J]
    // [M] [Z] [V] [S] [S] [V] [Q] [H] [M]
    // [W] [B] [H] [F] [L] [F] [J] [V] [B]
    //  1   2   3   4   5   6   7   8   9

    const NUM_STACKS: usize = 9;
    const NUM_LINES: usize = 8;

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); NUM_STACKS + 1];
    if let Ok(mut lines) = read_lines("assets/05.txt") {
        for _ in 1..=NUM_LINES {
            let chars: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
            // println!("{:?}", chars);
            for i in 0..NUM_STACKS {
                let c = chars[1 + 4 * i];
                if c.is_alphabetic() {
                    stacks[i + 1].push_back(c);
                }
            }
        }
        // println!("Start:\n{:?}", stacks);

        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split(' ').collect();
                if words.len() != 6 {
                    continue;
                }
                let [how_many, from, to]: [usize; 3] =
                    [words[1], words[3], words[5]].map(|x| x.parse().unwrap());

                for i in 0..how_many {
                    let c: char;
                    if part_two {
                        c = stacks[from].remove(how_many - i - 1).unwrap();
                    } else {
                        c = stacks[from].pop_front().unwrap();
                    }
                    stacks[to].push_front(c);
                }
            }
        }

        // println!("Finish:\n{:?}", stacks);

        for i in 1..=NUM_STACKS {
            print!("{}", stacks[i].pop_front().unwrap());
        }
        println!();
    }
}

fn main() {
    println!("Hello, advent!");

    p05(true);
    p05(false);
    p04(true);
    p04(false);
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
