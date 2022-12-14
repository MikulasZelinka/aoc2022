use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::str::{FromStr, Lines};
use std::string::ParseError;

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

                // part 1 – outside
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
use regex::Regex;

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

fn main() {
    println!("Hello, advent!");

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
