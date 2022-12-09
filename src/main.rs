use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
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

fn p09() {
    if let Ok(lines) = read_lines("assets/09.txt") {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        let (mut head_x, mut head_y) = (0, 0);
        let (mut tail_x, mut tail_y) = (0, 0);
        visited.insert((tail_x, tail_y));

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
                        "R" => head_x += 1,
                        "L" => head_x -= 1,
                        "U" => head_y += 1,
                        "D" => head_y -= 1,
                        _ => {}
                    }

                    let (mut diff_x, mut diff_y) = (0, 0);
                    if (tail_x + 1 < head_x) || (tail_x < head_x && ((tail_y - head_y).abs() >= 2))
                    {
                        diff_x += 1;
                    } else if (tail_x - 1 > head_x)
                        || (tail_x > head_x && ((tail_y - head_y).abs() >= 2))
                    {
                        diff_x -= 1;
                    }
                    if (tail_y + 1 < head_y) || (tail_y < head_y && ((tail_x - head_x).abs() >= 2))
                    {
                        diff_y += 1;
                    } else if (tail_y - 1 > head_y)
                        || (tail_y > head_y && ((tail_x - head_x).abs() >= 2))
                    {
                        diff_y -= 1;
                    }

                    tail_x += diff_x;
                    tail_y += diff_y;

                    visited.insert((tail_x, tail_y));
                }
            }
        }
        println!("{}", visited.len());
    }
}
fn main() {
    println!("Hello, advent!");

    p09();
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
