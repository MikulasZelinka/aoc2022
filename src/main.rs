use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::str::{FromStr, Lines};
use std::string::ParseError;

use indicatif::ParallelProgressIterator;
use itertools::{sorted, Itertools};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rust_lapper::{Interval, Lapper};
use serde_json::Value;
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

fn p06(part_two: bool) {
    if let Ok(mut lines) = read_lines("assets/06.txt") {
        // BWHY can't we use '?' instead of 'unwrap' here
        let line = lines.next().unwrap().unwrap();
        let mut bytes = line.bytes();
        let window_size: usize = if part_two { 14 } else { 4 };
        let mut window: VecDeque<u8> = VecDeque::new();
        for _ in 0..window_size {
            window.push_back(bytes.next().unwrap());
        }
        for (i, c) in bytes.enumerate() {
            if HashSet::<&u8>::from_iter(window.iter()).len() == window_size {
                println!("{}", i + window_size);
                return;
            }
            window.pop_front();
            window.push_back(c);
        }
    }
}

fn p07(part_two: bool) {
    let mut path: PathBuf = PathBuf::new();
    let mut sizes: HashMap<PathBuf, u32> = HashMap::new();

    if let Ok(lines) = read_lines("assets/07.txt") {
        for line in lines {
            if let Ok(line) = line {
                let l = line.as_str();
                match l {
                    "$ cd /" => {
                        path.clear();
                        path.push("/");
                    }
                    "$ cd .." => {
                        path.pop();
                    }
                    line if line.starts_with("$ cd ") => path.push(&line[5..]),

                    line if line.chars().next().unwrap().is_numeric() => {
                        let size: u32 = line.split(' ').next().unwrap().parse().unwrap();

                        let mut tmp_path = path.clone();

                        loop {
                            let last_size = match sizes.get(&tmp_path) {
                                None => 0,
                                Some(&v) => v,
                            };
                            sizes.insert(tmp_path.clone(), last_size + size);

                            if tmp_path.parent().is_none() {
                                break;
                            }
                            tmp_path.pop();
                        }
                    }

                    _ => (),
                }
            }
        }

        // println!("{:?}", sizes);

        if part_two {
            const TOTAL: u32 = 70000000;
            const REQUIRED: u32 = 30000000;

            let used = sizes.get(Path::new("/")).unwrap();
            let unused = TOTAL - used;

            let min_to_free_up = REQUIRED - unused;

            println!(
                "{}",
                sizes
                    .into_values()
                    .filter(|x| x > &min_to_free_up)
                    .min()
                    .unwrap()
            );
        } else {
            println!(
                "{}",
                sizes.into_values().filter(|x| x < &100_000).sum::<u32>()
            )
        }
    }
}

fn p08() {
    let mut grid: Vec<Vec<u8>> = vec![];

    fn walk_in_direction<I, J>(
        grid: &Vec<Vec<u8>>,
        outer: I,
        inner: J,
        num_rows: usize,
        num_cols: usize,
        cols_first: bool,
    ) -> (Vec<Vec<u8>>, Vec<Vec<usize>>)
    where
        I: IntoIterator<Item = usize>,
        // I: Clone,
        // J: IntoIterator<Item = usize>,
        J: Iterator<Item = usize>,
        J: Clone,
    {
        let mut outside_view = vec![vec![b'0' - 1; num_cols]; num_rows];
        let mut inside_view = vec![vec![0usize; num_cols]; num_rows];

        for r in outer {
            let mut max: u8 = b'0' - 1;
            let mut last_seen_x_or_higher = [0usize; 10];

            // BWHY is clone() necessary here?
            for (i, c) in inner.clone().enumerate() {
                let (row, col) = if cols_first { (c, r) } else { (r, c) };

                let x = grid[row][col];

                // part 2 - inside
                {
                    let x_value = (x - b'0') as usize;
                    inside_view[row][col] = i - last_seen_x_or_higher[x_value];
                    for k in 0..=x_value {
                        last_seen_x_or_higher[k] = i;
                    }
                }

                // part 1 ??? outside
                {
                    outside_view[row][col] = max;
                    if max == b'9' {
                        continue;
                    }
                    if max < x {
                        max = x;
                    }
                }
            }
        }
        (outside_view, inside_view)
    }

    if let Ok(lines) = read_lines("assets/08.txt") {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }
                grid.push(line.bytes().collect());
            }
        }

        let num_rows = grid.len();
        let num_cols = grid[0].len();

        let views = vec![
            // 0; left to right
            walk_in_direction(&grid, 0..num_rows, 0..num_cols, num_rows, num_cols, false),
            // 1; right to left
            walk_in_direction(
                &grid,
                0..num_rows,
                (0..num_cols).rev(),
                num_rows,
                num_cols,
                false,
            ),
            // 2; top to bottom
            walk_in_direction(&grid, 0..num_cols, 0..num_rows, num_rows, num_cols, true),
            // 3; bottom to top
            walk_in_direction(
                &grid,
                0..num_cols,
                (0..num_rows).rev(),
                num_rows,
                num_cols,
                true,
            ),
        ];

        // part 2
        {
            let mut max: usize = 0;
            for (r, row) in grid.iter().enumerate() {
                for (c, _) in row.iter().enumerate() {
                    let score = views.iter().fold(1, |acc, x| x.1[r][c] * acc);
                    if score > max {
                        max = score;
                    }
                }
            }
            println!("{max}");
        }

        // part 1
        {
            let mut visible: u32 = 0;
            for (r, row) in grid.iter().enumerate() {
                for (c, x) in row.iter().enumerate() {
                    let x = *x;
                    if x > views[0].0[r][c]
                        || x > views[1].0[r][c]
                        || x > views[2].0[r][c]
                        || x > views[3].0[r][c]
                    {
                        visible += 1;
                    }
                }
            }
            println!("{visible}");
        }
    }
}

fn p09(length: usize) {
    if let Ok(lines) = read_lines("assets/09.txt") {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        let mut knots = vec![(0i32, 0i32); length];

        visited.insert(knots[0]);

        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }

                let mut line = line.split(" ");
                let (dir, steps): (&str, i32) =
                    (line.next().unwrap(), line.next().unwrap().parse().unwrap());

                for _ in 0..steps {
                    match dir {
                        "R" => knots[0].0 += 1,
                        "L" => knots[0].0 -= 1,
                        "U" => knots[0].1 += 1,
                        "D" => knots[0].1 -= 1,
                        _ => {}
                    }

                    for i in 0..length - 1 {
                        let j = i + 1;

                        let (mut diff_x, mut diff_y) = (0, 0);
                        if (knots[j].0 + 1 < knots[i].0)
                            || (knots[j].0 < knots[i].0 && ((knots[j].1 - knots[i].1).abs() >= 2))
                        {
                            diff_x += 1;
                        } else if (knots[j].0 - 1 > knots[i].0)
                            || (knots[j].0 > knots[i].0 && ((knots[j].1 - knots[i].1).abs() >= 2))
                        {
                            diff_x -= 1;
                        }
                        if (knots[j].1 + 1 < knots[i].1)
                            || (knots[j].1 < knots[i].1 && ((knots[j].0 - knots[i].0).abs() >= 2))
                        {
                            diff_y += 1;
                        } else if (knots[j].1 - 1 > knots[i].1)
                            || (knots[j].1 > knots[i].1 && ((knots[j].0 - knots[i].0).abs() >= 2))
                        {
                            diff_y -= 1;
                        }

                        knots[j].0 += diff_x;
                        knots[j].1 += diff_y;
                    }

                    visited.insert(knots[length - 1]);
                }
            }
        }
        println!("{}", visited.len());
    }
}

fn p10() {
    if let Ok(mut lines) = read_lines("assets/10.txt") {
        let mut waiting = false;
        let mut x = 1i32;
        let mut buffer = 0i32;
        let mut signal_strength = 0i32;

        for cycle in 1..=240 {
            if (x - ((cycle % 40) - 1)).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            if cycle % 40 == 0 {
                println!();
            }
            if let 20 | 60 | 100 | 140 | 180 | 220 = cycle {
                signal_strength += x * cycle;
            }
            if waiting {
                x += buffer;
                waiting = false;
                continue;
            }

            let line = lines.next().unwrap().unwrap();
            let mut line = line.split(" ");
            let command = line.next().unwrap();
            match command {
                "noop" => continue,
                "addx" => {
                    waiting = true;
                    buffer = line.next().unwrap().parse().unwrap();
                }
                _ => {}
            }
        }
        println!("{}", signal_strength);
    }
}

use lazy_static::lazy_static;
use regex::{Match, Regex};

fn p11(num_rounds: usize, worry_div: usize) {
    #[derive(Debug, PartialEq)]
    struct Monkey {
        id: usize,
        items: VecDeque<usize>,
        op: char,
        arg: Option<usize>,
        test_div_by: usize,
        test_true_id: usize,
        test_false_id: usize,
        num_inspects: usize,
    }

    struct ItemThrow {
        target: usize,
        worry: usize,
    }

    impl Monkey {
        fn catch(&mut self, item: usize) {
            self.items.push_back(item);
        }

        fn inspect_and_throw(&mut self, worry_div: usize, lcm: usize) -> Vec<ItemThrow> {
            // for x in self.items.iter_mut() {
            //     *x = match self.op {
            //         '*' => (*x * self.arg.unwrap_or(*x)) / 3,
            //         '+' => (*x + self.arg.unwrap_or(*x)) / 3,
            //         _ => panic!(),
            //     };
            //     self.num_inspects += 1;
            // }
            let mut throws: Vec<ItemThrow> = Vec::new();

            for x in self.items.drain(..) {
                self.num_inspects += 1;

                let mut worry = match self.op {
                    '*' => x * self.arg.unwrap_or(x),
                    '+' => x + self.arg.unwrap_or(x),
                    _ => panic!(),
                };

                worry /= worry_div;

                // BWHY does this change output of part 1?
                worry %= lcm;

                throws.push(ItemThrow {
                    target: if worry % self.test_div_by == 0 {
                        self.test_true_id
                    } else {
                        self.test_false_id
                    },
                    worry,
                });
            }
            throws
        }
    }

    fn get_next_last_number(lines: &mut Lines) -> usize {
        lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }
    impl FromStr for Monkey {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            lazy_static! {
                static ref RE_NUM: Regex = Regex::new(r"(\d+)").unwrap();
            }

            // Monkey 0:
            // Starting items: 92, 73, 86, 83, 65, 51, 55, 93
            // Operation: new = old * 5
            // Test: divisible by 11
            //   If true: throw to monkey 3
            //   If false: throw to monkey 4

            let mut lines = s.lines();

            // Monkey 0:
            let id: usize = RE_NUM
                .find(lines.next().unwrap())
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            // Starting items: 92, 73, 86, 83, 65, 51, 55, 93
            let items: VecDeque<usize> = RE_NUM
                .find_iter(lines.next().unwrap())
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect();

            // Operation: new = old * 5
            let (op, arg): (char, Option<usize>) = {
                let l: Vec<&str> = lines.next().unwrap().split(" ").collect();
                let op = l[l.len() - 2].chars().next().unwrap();
                let arg = l[l.len() - 1];

                (
                    op,
                    match arg {
                        "old" => None,
                        _ => Some(arg.parse::<usize>().unwrap()),
                    },
                )
            };

            // Test: divisible by 11
            let test_div_by = get_next_last_number(&mut lines);

            //   If true: throw to monkey 3
            let test_true_id: usize = get_next_last_number(&mut lines);

            //   If false: throw to monkey 4
            let test_false_id: usize = get_next_last_number(&mut lines);

            Ok(Monkey {
                id,
                items,
                op,
                arg,
                test_div_by,
                test_true_id,
                test_false_id,
                num_inspects: 0,
            })
        }
    }

    let mut monkeys: Vec<Monkey> = vec![
        // Monkey {
        //     id: 0,
        //     items: vec![92, 73, 86, 83, 65, 51, 55, 93],
        //     // operation: |x| x * 5,
        //     op: 'x',
        //     arg: Some(5),
        //     test_div_by: 11,
        //     test_true_id: 3,
        //     test_false_id: 4,
        // }
    ];

    let mut monkey_str = String::new();

    let lines = read_lines("assets/11.txt").unwrap();
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                monkeys.push(Monkey::from_str(&monkey_str).unwrap());
                monkey_str.clear();
                continue;
            }
            monkey_str.push_str(&line);
            monkey_str.push('\n');
        }
    }
    // create last monkey if there's not enough newlines at the end of the file
    if !monkey_str.is_empty() {
        monkeys.push(Monkey::from_str(&monkey_str).unwrap());
        monkey_str.clear();
    }
    // println!("{:?}", monkeys);

    let lcm: usize = monkeys.iter().map(|x| x.test_div_by).product();

    for _ in 0..num_rounds {
        // BHOW to do this when we can't borrow monkeys twice? is t he for i in range really necessary?
        // for monkey in monkeys.iter_mut() {
        //     let throws = monkey.inspect_and_throw();
        //     for throw in throws {
        //         monkeys[throw.target].catch(throw.worry);
        //     }
        // }
        for i in 0..(&monkeys).len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let throws = monkey.inspect_and_throw(worry_div, lcm);
            for throw in throws {
                // BHOW do these two differ?
                monkeys[throw.target].catch(throw.worry);
                // monkeys.get_mut(throw.target).unwrap().catch(throw.worry);
            }
        }
    }

    let mut num_inspects = monkeys
        .iter()
        .map(|monkey| monkey.num_inspects)
        .collect::<Vec<usize>>();
    num_inspects.sort();

    let monkey_business: usize =
        num_inspects[num_inspects.len() - 1] * num_inspects[num_inspects.len() - 2];

    println!("p11: {}", monkey_business);
}

fn p12(part_two: bool) {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    struct Position {
        x: usize,
        y: usize,
    }

    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    let mut heightmap: Vec<Vec<u8>> = vec![];

    let mut all_starts: Vec<Position> = vec![];

    let lines = read_lines("assets/12.txt").unwrap();
    for (r, line) in lines.enumerate() {
        if let Ok(line) = line {
            if line.is_empty() {
                continue;
            }
            let mut row: Vec<u8> = vec![];

            for (c, x) in line.bytes().enumerate() {
                let mut height = x;
                if x == b'S' {
                    start = Position { x: c, y: r };
                    height = b'a';
                } else if x == b'E' {
                    end = Position { x: c, y: r };
                    height = b'z';
                }
                if height == b'a' {
                    all_starts.push(Position { x: c, y: r });
                }
                row.push(height);
            }
            heightmap.push(row);
        }
    }

    let y_max = heightmap.len() - 1;
    let x_max = heightmap[0].len() - 1;

    let starts = if part_two { all_starts } else { vec![start] };

    let mut visited: HashSet<Position> = HashSet::from_iter(starts.clone());
    let mut q: VecDeque<Position> = VecDeque::from_iter(starts.clone());
    let mut distances: HashMap<Position, usize> =
        HashMap::from_iter(starts.iter().map(|x| (*x, 0)));

    while !q.is_empty() {
        // p = previous, n = next
        let p = q.pop_front().unwrap();

        for n in [
            Position {
                x: (p.x + 1).min(x_max),
                y: p.y,
            },
            Position {
                x: p.x,
                y: (p.y + 1).min(y_max),
            },
            Position {
                x: p.x.saturating_sub(1),
                y: p.y,
            },
            Position {
                x: p.x,
                y: p.y.saturating_sub(1),
            },
        ] {
            if heightmap[n.y][n.x] > heightmap[p.y][p.x] + 1 || visited.contains(&n) {
                continue;
            }
            q.push_back(n);
            visited.insert(n);
            distances.insert(n, distances.get(&p).unwrap() + 1);
            if n == end {
                println!("{}", distances.get(&end).unwrap());
                return;
            }
        }
    }
}

fn p13() {
    let mut lines = read_lines("assets/13.txt").unwrap();

    lazy_static! {
        static ref RE_NUM: Regex = Regex::new(r"(\d+)").unwrap();
    }

    let mut index: usize = 0;
    let mut sum: usize = 0;

    fn compare(left: Value, right: Value) -> Option<bool> {
        // debugging watches
        // let l_s = serde_json::to_string(&left).unwrap();
        // let r_s = serde_json::to_string(&right).unwrap();

        // let l_a = left.is_array();
        // let r_a = right.is_array();

        // let l_u = left.is_u64();
        // let r_u = right.is_u64();

        if left.is_array() && !right.is_array() {
            return compare(left, Value::Array(vec![right]));
        }
        if !left.is_array() && right.is_array() {
            return compare(Value::Array(vec![left]), right);
        }
        if left.is_array() && right.is_array() {
            let left_a = left.as_array().unwrap();
            let right_a = right.as_array().unwrap();

            for (l, r) in zip(left_a.clone(), right_a.clone()) {
                match compare(l, r) {
                    None => {}
                    Some(result) => return Some(result),
                }
            }
            if left_a.len() > right_a.len() {
                return Some(false);
            } else if left_a.len() < right_a.len() {
                return Some(true);
            }
            return None;
        }

        // both must be is_u64()
        if left.as_u64().unwrap() < right.as_u64().unwrap() {
            return Some(true);
        } else if left.as_u64().unwrap() > right.as_u64().unwrap() {
            return Some(false);
        }
        None
    }

    let mut packets: Vec<Value> = vec![];

    loop {
        index += 1;

        let l1 = lines.next();
        let l2 = lines.next();

        lines.next();
        if l1.is_none() || l2.is_none() {
            break;
        }
        let l1 = l1.unwrap().unwrap();
        let l2 = l2.unwrap().unwrap();

        let l: Value = serde_json::from_str(&l1).unwrap();
        let r: Value = serde_json::from_str(&l2).unwrap();

        packets.push(l.clone());
        packets.push(r.clone());

        if compare(l, r).unwrap_or(true) {
            sum += index;
        }
    }

    let divider1: Value = serde_json::from_str("[[2]]").unwrap();
    let divider2: Value = serde_json::from_str("[[6]]").unwrap();

    // BWHY is all this cloning necessary, is there a better way if Value doesn't have Copy?
    let num_smaller1 = packets
        .iter()
        .cloned()
        .filter(|x| compare(x.clone(), divider1.clone()).unwrap_or(true))
        .count();
    let num_smaller2 = packets
        .iter()
        .cloned()
        .filter(|x| compare(x.clone(), divider2.clone()).unwrap_or(true))
        .count();

    // part 2:
    println!("{}", (num_smaller1 + 1) * (num_smaller2 + 2));

    // part 1:
    println!("{}", sum);
}

fn p14(part_two: bool) {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    struct Position {
        x: usize,
        y: usize,
    }

    lazy_static! {
        static ref RE_TWO_NUMS: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
    }

    // let mut segments: HashSet<(Position, Position)> = HashSet::new();
    let mut occupied: HashSet<Position> = HashSet::new();
    let mut y_max: usize = 0;

    for line in read_lines("assets/14.txt").unwrap().map(|x| x.unwrap()) {
        if line.is_empty() {
            continue;
        }

        let mut last_segment: Option<Position> = None;
        for caps in RE_TWO_NUMS.captures_iter(&line) {
            let current: Position = Position {
                x: caps.get(1).unwrap().as_str().parse().unwrap(),
                y: caps.get(2).unwrap().as_str().parse().unwrap(),
            };

            y_max = y_max.max(current.y);

            if let Some(last) = last_segment {
                // segments.insert((last, current));
                let (from, to) = if last.x < current.x || last.y < current.y {
                    (last, current)
                } else {
                    (current, last)
                };
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        occupied.insert(Position { x, y });
                    }
                }
            }
            last_segment = Some(current);
        }
    }

    if part_two {
        // for x in 500 - y_max + 2..=500 + y_max + 2 {
        for x in 0..=1000 {
            occupied.insert(Position { x, y: y_max + 2 });
        }
    }

    const START: Position = Position { x: 500, y: 0 };

    let mut last: Option<Position> = None;
    let mut saved = 0;
    let mut total = 0;

    let mut grains = 0;
    'outer: loop {
        let mut p = match last {
            None => START,
            Some(last) => last,
        };

        saved += p.y;

        last = None;

        loop {
            if !part_two && p.y >= y_max {
                break 'outer;
            }

            let space_left = !occupied.contains(&Position {
                x: p.x - 1,
                y: p.y + 1,
            });
            let space_down = !occupied.contains(&Position { x: p.x, y: p.y + 1 });
            let space_right = !occupied.contains(&Position {
                x: p.x + 1,
                y: p.y + 1,
            });

            if (space_left && space_down)
                || (space_left && space_right)
                || (space_down && space_right)
            {
                // if we have at least two options, we can start here next time
                last = Some(p);
            }

            if space_down {
                p.y += 1;
            } else if space_left {
                p.x -= 1;
                p.y += 1;
            } else if space_right {
                p.x += 1;
                p.y += 1;
            } else {
                // part 2
                if part_two && occupied.contains(&p) {
                    break 'outer;
                }
                total += p.y;
                occupied.insert(p);

                break;
            }
        }
        grains += 1;
    }

    // println!("{:?}", occupied);
    println!(
        "p14 grains: {}, saved {} steps out of {} (saved {:.1}%)",
        grains,
        saved,
        total,
        ((saved as f32) * 100.0 / total as f32)
    );
}

fn p15(test: bool) {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    struct Position {
        x: u32,
        y: u32,
    }

    impl Position {
        fn distance(&self, other: Position) -> u32 {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
        }
    }

    fn i_to_u(x: i32) -> u32 {
        // (x as i64 + ((u32::MAX / 2) as i64)) as u32
        (x as i64 + (1_000_000_000 as i64)) as u32
    }

    fn u_to_i(x: u32) -> i64 {
        x as i64 - (1_000_000_000)
    }

    fn cap_to_u(x: Option<Match>) -> u32 {
        let x: i32 = x.unwrap().as_str().parse().unwrap();
        i_to_u(x)
    }

    let y_target = if test {
        i_to_u(10i32)
    } else {
        i_to_u(2000000i32)
    };
    let (x_min, y_min) = (i_to_u(0), i_to_u(0));

    let (x_max, y_max) = if test {
        (i_to_u(20), i_to_u(20))
    } else {
        (i_to_u(4000000), i_to_u(4000000))
    };

    lazy_static! {
        static ref RE_TWO_NUMS: Regex =
            Regex::new(r"x=([0-9-]+), y=([0-9-]+).+x=([0-9-]+), y=([0-9-]+)").unwrap();
    }

    let mut covered: Vec<Interval<u32, u8>> = vec![];
    let mut beacons_x_covering: HashSet<u32> = HashSet::new();

    // part 2:
    let mut candidates: HashSet<Position> = HashSet::new();
    let mut distances: HashMap<Position, u32> = HashMap::new();

    for line in read_lines(if test {
        "assets/15_test.txt"
    } else {
        "assets/15.txt"
    })
    .unwrap()
    .map(|x| x.unwrap())
    {
        if line.is_empty() {
            continue;
        }

        let caps = RE_TWO_NUMS.captures(&line).unwrap();

        let (sensor, beacon) = (
            Position {
                x: cap_to_u(caps.get(1)),
                y: cap_to_u(caps.get(2)),
            },
            Position {
                x: cap_to_u(caps.get(3)),
                y: cap_to_u(caps.get(4)),
            },
        );

        if beacon.y == y_target {
            beacons_x_covering.insert(beacon.x);
        }

        let d = sensor.distance(beacon);
        distances.insert(sensor, d);

        // part 2
        {
            for x in 0..=d + 1 {
                let y = d + 1 - x;
                candidates.insert(Position {
                    x: sensor.x + x,
                    y: sensor.y + y,
                });
                candidates.insert(Position {
                    x: sensor.x + x,
                    y: sensor.y - y,
                });
                candidates.insert(Position {
                    x: sensor.x - x,
                    y: sensor.y + y,
                });
                candidates.insert(Position {
                    x: sensor.x - x,
                    y: sensor.y - y,
                });
            }
        }

        let y_diff = sensor.y.abs_diff(y_target);

        if d < y_diff {
            continue;
        }

        let spread = d - y_diff;
        covered.push(Interval {
            start: sensor.x - spread,
            stop: sensor.x + spread + 1,
            val: 0,
        });
    }

    // part 2:
    {
        'outer: for c in candidates.iter() {
            if c.x < x_min || c.y < y_min || c.x > x_max || c.y > y_max {
                continue;
            }
            for (sensor, distance) in distances.iter() {
                if c.distance(*sensor) <= *distance {
                    continue 'outer;
                }
            }
            let (x, y) = (u_to_i(c.x), u_to_i(c.y));
            println!("{}", x * 4000000 + y);
            break;
        }
    }

    // part 1:
    {
        println!(
            "{}",
            Lapper::new(covered).cov() - beacons_x_covering.len() as u32
        );
    }
}

fn p16(num_rounds: usize, num_explorers: usize, test: bool) {
    // struct Node<'a> {
    type NodeID = u8;

    struct Node {
        // id: NodeID,
        flow: u32,
        // edges: Vec<&'a Node<'a>>,
        edges: Vec<NodeID>,
    }

    // #[derive(Clone)]
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct SearchState {
        current_node: Vec<NodeID>,
        flow_per_round: u32,
        to_open: Vec<bool>,
        score: u32,
        last_node: Vec<Option<NodeID>>,
        history: Vec<NodeID>,
    }

    type StateHistory = (Vec<NodeID>, Vec<bool>);

    #[derive(Clone)]
    struct StateDelta {
        new_node: Option<NodeID>,
        last_node: Option<NodeID>,
        newly_opened: Option<NodeID>,
    }

    lazy_static! {
        static ref RE_ROW: Regex =
            Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
        static ref RE_VALVES: Regex = Regex::new(r"([A-Z][A-Z])").unwrap();
    }

    let mut nodes: Vec<Node> = vec![];
    let mut id_to_name: Vec<String> = vec![];
    let mut name_to_id: HashMap<String, NodeID> = HashMap::new();
    let mut to_open: Vec<bool> = vec![];
    let mut edge_names: Vec<Vec<String>> = vec![];
    let mut start: NodeID = 0;

    for (i, line) in read_lines(if test {
        "assets/16_test.txt"
    } else {
        "assets/16.txt"
    })
    .unwrap()
    .map(|x| x.unwrap())
    .enumerate()
    {
        let i = i as NodeID;
        if line.is_empty() {
            continue;
        }

        // println!("line: {}", &line);
        let caps = RE_ROW.captures(&line).unwrap();
        let valve = caps.get(1).unwrap().as_str();
        let flow: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let edges = caps.get(3).unwrap().as_str();

        if valve == "AA" {
            start = i;
        }

        id_to_name.push(valve.to_string());
        name_to_id.insert(valve.to_string(), i);

        to_open.push(if flow > 0 { true } else { false });

        edge_names.push(
            RE_VALVES
                .captures_iter(edges)
                .map(|x| x.get(1).unwrap().as_str().to_string())
                .collect(),
        );

        // println!("{:?}", edges);

        // println!();

        nodes.push(Node {
            flow,
            edges: vec![],
        });
    }

    for (i, node_edges) in edge_names.iter().enumerate() {
        for edge_name in node_edges.iter() {
            // BWHY is the "let _" recommended?
            let _ = &nodes.get_mut(i).unwrap().edges.push(name_to_id[edge_name]);
        }
    }

    let num_nodes = nodes.len();
    let mut distances: Vec<Vec<i32>> = vec![vec![0; num_nodes]; num_nodes];
    for i in 0..nodes.len() {
        for j in 0..i {
            let mut distance = -1;

            // BFS
            {
                let mut bfs_distances: HashMap<NodeID, i32> = HashMap::from([(i as NodeID, 0)]);
                let mut visited: HashSet<NodeID> = HashSet::from([i as NodeID]);
                let mut q: VecDeque<NodeID> = VecDeque::from([i as NodeID]);

                'bfs: while !q.is_empty() {
                    let from = q.pop_front().unwrap();

                    for to in &nodes.get(i).unwrap().edges {
                        if visited.contains(&to) {
                            continue;
                        }
                        q.push_back(*to);
                        visited.insert(*to);
                        let d = bfs_distances.get(&from).unwrap() + 1;
                        if *to == j as NodeID {
                            distance = d;
                            break 'bfs;
                        }
                        bfs_distances.insert(*to, d);
                    }
                }
            }
            distances[i][j] = distance;
            distances[j][i] = distance;
        }
    }

    // adapted from https://stackoverflow.com/questions/70050040/fast-idiomatic-floyd-warshall-algorithm-in-rust
    fn floyd_warshall(dist: &mut Vec<Vec<i32>>) {
        let n = dist.len();
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if dist[j][i] < 0 || dist[i][k] < 0 {
                        // infinite distance, don't use it
                        continue;
                    } else if dist[j][k] < 0 {
                        // always override if we had infinite before
                        dist[j][k] = i32::MAX;
                    }
                    dist[j][k] = std::cmp::min(dist[j][k], dist[j][i] + dist[i][k]);
                }
            }
        }
    }
    floyd_warshall(&mut distances);

    let state = SearchState {
        current_node: vec![start; num_explorers],
        flow_per_round: 0,
        to_open: to_open.clone(),
        score: 0,
        last_node: vec![None; num_explorers],
        history: vec![],
    };

    let mut q_old: Vec<SearchState> = Vec::from([state]);
    let mut states_visited: HashMap<StateHistory, u32> = HashMap::new();

    fn expected_total_score(
        score: u32,
        flow_per_round: u32,
        num_rounds: usize,
        round: usize,
    ) -> u32 {
        score + (flow_per_round * (num_rounds - round) as u32)
    }

    const MAX_QUEUE_SIZE: usize = 1_000;

    for round in 1..=num_rounds {
        // println!("Round {:02}", round);
        // println!("- queue init: {}", q_old.len());

        let mut q_new: Vec<SearchState> = Vec::new();
        // let mut distance_skips: u32 = 0;
        // let mut distance_conts: u32 = 0;

        // TODO - ugly hack, just keep the MAX_QUEUE_SIZE most promising
        // otherwise, the queue_size explodes to 10s of millions in p2, real input
        // and keeping just the best 1_000 seemed to work just fine ??\_(???)_/??
        if q_old.len() > MAX_QUEUE_SIZE {
            q_old.sort_unstable_by_key(|x| {
                u32::MAX - expected_total_score(x.score, x.flow_per_round, num_rounds, round)
            });
            q_old.truncate(MAX_QUEUE_SIZE);
        }

        for s in q_old.iter() {
            let mut s_new = s.clone();
            s_new.score += s_new.flow_per_round;

            // 1 - done opening, and thus also searching
            if s_new.to_open.iter().all(|x| *x == false) {
                q_new.push(s_new);
                continue;
            }

            let to_open_list = s_new
                .to_open
                .iter()
                .enumerate()
                .filter_map(|(index, &r)| (r == true).then(|| index))
                .collect::<Vec<_>>();

            let mut explorer_deltas: Vec<Vec<StateDelta>> = vec![];

            for (i_exp, node) in s_new.current_node.iter().enumerate() {
                let node = *node;
                let node_i = node as usize;

                let mut q_node: Vec<StateDelta> = vec![];

                // 2 - open:
                if s_new.to_open[node_i] {
                    q_node.push(StateDelta {
                        new_node: None,
                        last_node: None,
                        newly_opened: Some(node),
                    });
                }

                // 3 - search
                for next in nodes[node_i].edges.iter() {
                    if let Some(last) = s_new.last_node[i_exp] {
                        if &last == next {
                            continue;
                        }
                    }

                    let mut gets_closer_to_opening = false;

                    // BHOW to dereference inline in the for ..iter()?
                    for next_to_open in to_open_list.iter() {
                        if distances[*next as usize][*next_to_open]
                            < distances[node_i][*next_to_open]
                        {
                            gets_closer_to_opening = true;

                            break;
                        }
                    }
                    if !gets_closer_to_opening {
                        // distance_skips += 1;
                        continue;
                    }
                    // distance_conts += 1;

                    q_node.push(StateDelta {
                        new_node: Some(*next),
                        last_node: Some(node),
                        newly_opened: None,
                    });
                }
                explorer_deltas.push(q_node)
            }

            let state_delta_combinations = explorer_deltas.iter().multi_cartesian_product();
            for state_deltas in state_delta_combinations {
                let newly_opened: Vec<NodeID> = state_deltas
                    .iter()
                    .filter_map(|&x| x.newly_opened)
                    .collect();
                // if has duplicates, skip - each node can only be opened by one explorer
                if newly_opened.len() > HashSet::<&NodeID>::from_iter(newly_opened.iter()).len() {
                    continue;
                }

                let mut s_new_combined = s_new.clone();
                for (i, state_delta) in state_deltas.iter().enumerate() {
                    // update current node if relevant
                    if let Some(new) = state_delta.new_node {
                        s_new_combined.current_node[i] = new;
                    }
                    // update last node always
                    s_new_combined.last_node[i] = state_delta.last_node;

                    // handle newly opened completely
                    if let Some(opened) = state_delta.newly_opened {
                        s_new_combined.to_open[opened as usize] = false;
                        s_new_combined.flow_per_round += nodes[opened as usize].flow;
                        s_new_combined.history.push(opened);
                    }
                }

                // the order of explorers doesn't matter, and we sort them for better deduplication
                let state_history: StateHistory = (
                    sorted(s_new_combined.current_node.clone()).collect(),
                    s_new_combined.to_open.clone(),
                );

                let (s_fpr, s_score) = (s_new_combined.flow_per_round, s_new_combined.score);

                let s_total = expected_total_score(s_score, s_fpr, num_rounds, round);

                // BHOW to simplify this?
                match states_visited.get(&state_history) {
                    Some(&old_total) => {
                        if old_total >= s_total {
                            continue;
                        }
                    }
                    None => {}
                }

                states_visited.insert(state_history, s_total);

                q_new.push(s_new_combined);
            }
        }

        q_old = q_new;
        // println!("- queue  new: {}", q_old.len());
        // println!("- states met: {}", states_visited.len());
        // println!("- dist skips: {}", distance_skips);
        // println!("- dist conts: {}", distance_conts);
        // println!();
    }

    let best = q_old.iter().max_by_key(|x| x.score).unwrap();

    println!(
        "{}, history: {:?}",
        best.score,
        best.history
            .iter()
            .map(|x| id_to_name[(*x) as usize].clone())
            .collect::<Vec<String>>()
    );
}

fn p17(test: bool, num_rocks: u64) {
    const EMPTY: u8 = 0b00000000;
    const FULL: u8 = 0b11111110;
    const LEFT: u8 = 0b10000000;
    const RIGHT: u8 = 0b00000010;

    const ROCK_HEIGHT: usize = 4;
    type ROCK = [u8; ROCK_HEIGHT];

    const NUM_ROCKS: usize = 5;
    #[rustfmt::skip]
    const ROCKS: [ROCK; NUM_ROCKS] = [
        // empty, empty, <shape * 5>, empty
        [0b00111100, EMPTY, EMPTY, EMPTY],
        [
            0b00010000,
            0b00111000,
            0b00010000, EMPTY
        ],
        [
            0b00111000,
            0b00001000,
            0b00001000, EMPTY
        ],
        [
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000
        ],
        [
            0b00110000,
            0b00110000, EMPTY, EMPTY
        ],
    ];

    const ROCK_HEIGHTS: [usize; NUM_ROCKS] = [1, 3, 3, 4, 2];

    const VERTICAL_SPACING: usize = 3;

    fn shift(x: u8, dx: i8) -> u8 {
        if dx > 0 {
            x >> dx
        } else if dx < 0 {
            x << -dx
        } else {
            x
        }
    }

    fn next_to_wall(rock: &ROCK, dx: i8, side: u8) -> bool {
        rock.iter().any(|x| shift(*x, dx) & side > 0)
    }

    fn collides(rock: &ROCK, dx: i8, chamber: &[u8]) -> bool {
        rock.iter()
            .zip_eq(chamber)
            .any(|(x, y)| shift(*x, dx) & y > 0)
    }

    let mut rocks = ROCKS.iter().cycle();
    let mut rock_heights = ROCK_HEIGHTS.iter().cycle();

    // BWHY do we need to have two assignments here (input and moves)?
    let input = read_lines(if test {
        "assets/17_test.txt"
    } else {
        "assets/17.txt"
    })
    .unwrap()
    .next()
    .unwrap()
    .unwrap();

    let mut moves = input.bytes().cycle();

    let mut tower_height: usize = 1;

    let mut chamber: Vec<u8> = vec![FULL];

    let mut cycle_i = 0;

    type BlockDistances = [usize; 7];
    type HeightRound = (u64, u64);

    let mut states: HashMap<BlockDistances, HeightRound> = HashMap::new();

    let mut found_cycle = false;
    let mut rocks_left_after_cycle = 0;
    let mut bonus_height_from_cycle = 0;

    // for _k in tqdm!(1..=num_rocks) {
    for rock_i in 1..=num_rocks {
        let rock: &ROCK = rocks.next().unwrap();
        let height = *rock_heights.next().unwrap();
        // let mut height: usize = rock.map(|x| (x & 1) as usize).iter().sum();

        let mut current_piece_base = tower_height + VERTICAL_SPACING;

        let ceiling = current_piece_base + ROCK_HEIGHT;
        if chamber.len() < ceiling {
            chamber.extend(vec![EMPTY; ceiling - chamber.len()]);
        }

        let mut dx = 0;
        loop {
            let (from, to) = (current_piece_base, current_piece_base + ROCK_HEIGHT);

            if !found_cycle && cycle_i == input.len() {
                cycle_i = 0;

                let mut new_state: [usize; 7] = [0; 7];
                for i in 0..7 {
                    let mut spaces = 0;
                    loop {
                        let x = chamber[tower_height - spaces];

                        if ((x >> (7 - i)) & 1) != 0 {
                            new_state[i] = spaces;
                            break;
                        }
                        spaces += 1;
                    }
                }
                if let Some((state, (last_height, last_round))) = states.get_key_value(&new_state) {
                    println!(
                        "Match on {:?}, th: {}, round {}, h diff: {}, k diff: {}",
                        state,
                        tower_height,
                        rock_i,
                        tower_height as u64 - last_height,
                        rock_i - last_round,
                    );

                    found_cycle = true;

                    // TODO: this seems to give an almost correct answer
                    // if "x" is correct, this also gave "x - 3" and "x - 2",
                    // probably a single rock error
                    let rock_cycle_length = rock_i - last_round;
                    let rocks_left = num_rocks - rock_i;
                    let rock_cycles_left = rocks_left / rock_cycle_length;
                    rocks_left_after_cycle = rocks_left - (rock_cycles_left * rock_cycle_length);
                    bonus_height_from_cycle =
                        rock_cycles_left * (tower_height as u64 - last_height);
                }

                states.insert(new_state, (tower_height as u64, rock_i));
            }

            cycle_i += 1;

            // wind
            match moves.next().unwrap() {
                b'<' => {
                    if !next_to_wall(&rock, dx, LEFT)
                        && !collides(&rock, dx - 1, &chamber[from..to])
                    {
                        dx -= 1;
                    }
                }
                b'>' => {
                    if !next_to_wall(&rock, dx, RIGHT)
                        && !collides(&rock, dx + 1, &chamber[from..to])
                    {
                        dx += 1;
                    }
                }
                _ => panic!(),
            }

            // fall
            if collides(&rock, dx, &chamber[from - 1..to - 1]) {
                for (rock_i, j) in (from..to).enumerate() {
                    chamber[j] |= shift(rock[rock_i], dx);
                }
                tower_height = tower_height.max(from + height);

                // debug print:
                // println!("{}", k);
                // for row in chamber.iter().rev() {
                //     println!("{:08b}", *row);
                // }
                // println!();

                break;
            }
            current_piece_base -= 1;
        }

        if rocks_left_after_cycle == 1 {
            break;
        }
        if rocks_left_after_cycle > 0 {
            rocks_left_after_cycle -= 1;
        }
    }
    println!(
        "height: {}",
        tower_height as u64 + bonus_height_from_cycle - 1
    );
}

fn p18(test: bool) {
    type Cube = (i32, i32, i32);

    let mut cubes: Vec<Cube> = vec![];

    for line in read_lines(if test {
        "assets/18_test.txt"
    } else {
        "assets/18.txt"
    })
    .unwrap()
    .map(|x| {
        x.unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple::<Cube>()
    })
    .map(|x| x.unwrap())
    {
        cubes.push(line);
    }
    let cube_set: HashSet<&Cube> = HashSet::from_iter(cubes.iter());

    let mut covered = 0;
    let num_cubes = cubes.len();
    for i in 0..num_cubes {
        for j in i + 1..num_cubes {
            let (c1, c2) = (cubes[i], cubes[j]);

            if (c1.0 == c2.0 && c1.1 == c2.1 && c1.2.abs_diff(c2.2) == 1)
                || (c1.0 == c2.0 && c1.2 == c2.2 && c1.1.abs_diff(c2.1) == 1)
                || (c1.1 == c2.1 && c1.2 == c2.2 && c1.0.abs_diff(c2.0) == 1)
            {
                covered += 2;
            }
        }
    }

    let min_cube = (
        cubes.iter().map(|x| x.0).min().unwrap() - 1,
        cubes.iter().map(|x| x.1).min().unwrap() - 1,
        cubes.iter().map(|x| x.2).min().unwrap() - 1,
    );
    let max_cube = (
        cubes.iter().map(|x| x.0).max().unwrap() + 1,
        cubes.iter().map(|x| x.1).max().unwrap() + 1,
        cubes.iter().map(|x| x.2).max().unwrap() + 1,
    );

    let mut stack = vec![min_cube];
    let mut visited = HashSet::from([min_cube]);
    let mut outside = 0;
    while let Some(last) = stack.pop() {
        for next in [
            (last.0 + 1, last.1, last.2),
            (last.0 - 1, last.1, last.2),
            (last.0, last.1 + 1, last.2),
            (last.0, last.1 - 1, last.2),
            (last.0, last.1, last.2 + 1),
            (last.0, last.1, last.2 - 1),
        ] {
            if next.0 < min_cube.0
                || next.1 < min_cube.1
                || next.2 < min_cube.2
                || next.0 > max_cube.0
                || next.1 > max_cube.1
                || next.2 > max_cube.2
            {
                continue;
            }

            if visited.contains(&next) {
                continue;
            }
            if cube_set.contains(&next) {
                outside += 1;
                continue;
            }
            visited.insert(next);
            stack.push(next);
        }
    }

    println!("day 18, p1: {}, p2: {}", num_cubes * 6 - covered, outside,);
}

fn p19(test: bool) {
    // TODO: better abstraction with less duplication
    // enum Resource {
    //     Ore,
    //     Clay,
    //     Obsidian,
    //     Geode,
    // }

    struct State {
        ore: i32,
        clay: i32,
        obsidian: i32,
        geode: i32,
        ore_r: i32,
        clay_r: i32,
        obsidian_r: i32,
        geode_r: i32,
    }

    impl State {
        fn new() -> Self {
            Self {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                ore_r: 1,
                clay_r: 0,
                obsidian_r: 0,
                geode_r: 0,
            }
        }

        fn next(&self) -> Self {
            Self {
                ore: self.ore + self.ore_r,
                clay: self.clay + self.clay_r,
                obsidian: self.obsidian + self.obsidian_r,
                geode: self.geode + self.geode_r,
                ..*self
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Blueprint {
        id: i32,
        ore_ore: i32,
        clay_ore: i32,
        obsidian_ore: i32,
        obsidian_clay: i32,
        geode_ore: i32,
        geode_obsidian: i32,
    }

    impl Blueprint {
        fn new(line: &str) -> Self {
            lazy_static! {
                static ref RE_BLUEPRINT: Regex =
                    Regex::new(r"Blueprint (?P<b>\d+): Each ore robot costs (?P<r_r>\d+) ore. Each clay robot costs (?P<c_r>\d+) ore. Each obsidian robot costs (?P<o_r>\d+) ore and (?P<o_c>\d+) clay. Each geode robot costs (?P<g_r>\d+) ore and (?P<g_o>\d+) obsidian.").unwrap();
            }

            // println!("{line}");
            let caps = RE_BLUEPRINT.captures_iter(line).next().unwrap();

            Self {
                id: caps["b"].parse().unwrap(),
                ore_ore: caps["r_r"].parse().unwrap(),
                clay_ore: caps["c_r"].parse().unwrap(),
                obsidian_ore: caps["o_r"].parse().unwrap(),
                obsidian_clay: caps["o_c"].parse().unwrap(),
                geode_ore: caps["g_r"].parse().unwrap(),
                geode_obsidian: caps["g_o"].parse().unwrap(),
            }
        }

        fn quality(&self, state: State, mins_left: i32, could_have_bought: [bool; 4]) -> i32 {
            let s = state.next();
            if mins_left == 1 {
                return s.geode;
            }

            // can't build more geode robots
            {
                if self.geode_obsidian >
                // (state.obsidian + ((mins_left - 2) * state.obsidian_r))
                 (state.obsidian + ((mins_left - 2) * (state.obsidian_r + mins_left - 2)))
                {
                    // println!("trigger in {}", mins_left);
                    return state.geode + (state.geode_r * mins_left);
                }
            }

            let can_build = (
                state.ore >= self.ore_ore,
                state.ore >= self.clay_ore,
                state.ore >= self.obsidian_ore && state.clay >= self.obsidian_clay,
                state.ore >= self.geode_ore && state.obsidian >= self.geode_obsidian,
            );
            match can_build {
                (false, false, false, false) => self.quality(s, mins_left - 1, [false; 4]),

                // always build last stage - geode - this is wrong
                // (.., true) => self.quality(
                //     State {
                //         ore: s.geode - self.geode_ore,
                //         obsidian: s.obsidian - self.geode_obsidian,
                //         geode_r: s.geode_r + 1,
                //         ..s
                //     },
                //     mins_left - 1,
                //     [false; 4],
                // ),
                _ => {
                    let mut options: Vec<i32> = vec![];

                    if !can_build.0 || !can_build.1 || !can_build.2 || !can_build.3 {
                        options.push(self.quality(
                            state.next(),
                            mins_left - 1,
                            [can_build.0, can_build.1, can_build.2, can_build.3],
                        ));
                    }
                    // let mut options: Vec<i32> = vec![];
                    if can_build.0
                        && !could_have_bought[0]
                        && ((self
                            .ore_ore
                            .max(self.clay_ore)
                            .max(self.obsidian_ore)
                            .max(self.geode_ore)
                            * (mins_left - 1))
                            >= (state.ore + state.ore_r * (mins_left - 1)))
                    {
                        options.push(self.quality(
                            State {
                                ore: s.ore - self.ore_ore,
                                ore_r: s.ore_r + 1,
                                ..s
                            },
                            mins_left - 1,
                            [false; 4],
                        ))
                    }
                    if can_build.1
                        && !could_have_bought[1]
                        && ((self.obsidian_clay * (mins_left - 1))
                            >= (state.clay + state.clay_r * (mins_left - 1)))
                    {
                        options.push(self.quality(
                            State {
                                ore: s.ore - self.clay_ore,
                                clay_r: s.clay_r + 1,
                                ..s
                            },
                            mins_left - 1,
                            [false; 4],
                        ))
                    }
                    if can_build.2
                        && !could_have_bought[2]
                        && ((self.geode_obsidian * (mins_left - 1))
                            >= (state.obsidian + state.obsidian_r * (mins_left - 1)))
                    {
                        options.push(self.quality(
                            State {
                                ore: s.ore - self.obsidian_ore,
                                clay: s.clay - self.obsidian_clay,
                                obsidian_r: s.obsidian_r + 1,
                                ..s
                            },
                            mins_left - 1,
                            [false; 4],
                        ))
                    }
                    if can_build.3 && !could_have_bought[3] {
                        options.push(self.quality(
                            State {
                                ore: s.ore - self.geode_ore,
                                obsidian: s.obsidian - self.geode_obsidian,
                                geode_r: s.geode_r + 1,
                                ..s
                            },
                            mins_left - 1,
                            [false; 4],
                        ))
                    }

                    *options.iter().max().unwrap_or(&0)
                }
            }
            // 10 * self.id
        }
    }

    let blueprints = read_lines(if test {
        "assets/19_test.txt"
    } else {
        "assets/19.txt"
    })
    .unwrap()
    .map(|x| Blueprint::new(x.unwrap().as_str()))
    .collect_vec();

    // println!("{:?}", blueprints);
    let b2 = blueprints.clone();

    let geodes = blueprints
        .par_iter()
        .progress_count(blueprints.len() as u64)
        .map(|x| x.quality(State::new(), 24, [false; 4]))
        .collect::<Vec<i32>>();
    println!("{:?}", geodes);

    println!(
        "{:?}",
        geodes
            .iter()
            .zip_eq(blueprints)
            .map(|(g, b)| g * b.id)
            .sum::<i32>()
    );
    println!();

    let geodes = b2[..3.min(b2.len())]
        .par_iter()
        .progress_count(b2.len() as u64)
        .map(|x| x.quality(State::new(), 32, [false; 4]))
        .collect::<Vec<i32>>();
    println!("{:?}, {}", geodes, geodes.iter().product::<i32>());
}

fn main() {
    println!("Hello, advent!");

    p19(true);
    p19(false);
    return;

    p18(false);
    p18(true);
    // {
    //     // 1525364431487 is correct
    //     // 1525364431488 is too high - it's a minor error based on the current rock height, I guess?
    //     // 1525364431485 is too low
    //     p17(false, 1000000000000);
    // }
    // p17(true, 1000000000000);
    p17(false, 2022);
    // p17(true, 2022);
    // p16(26, 2, false);
    p16(26, 2, true);
    // p16(30, 1, false);
    p16(30, 1, true);
    // p15(false); // takes ~20 seconds in release
    p15(true);
    p14(true);
    p14(false);
    p13();
    p12(true);
    p12(false);
    p11(10_000, 1);
    p11(20, 3);
    p10();
    p09(10);
    p09(2);
    p08();
    p07(true);
    p07(false);
    p06(true);
    p06(false);
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
