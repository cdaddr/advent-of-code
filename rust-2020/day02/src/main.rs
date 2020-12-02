#[macro_use] extern crate lazy_static;
use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input/day02").unwrap();
    let ans = part1(&input);
    println!("part1: {}", ans);

    let ans = part2(&input);
    println!("part2: {}", ans)
}

fn parse(line: &str) -> (String, i64, i64, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (\w+)$").unwrap();
    }

    let cap = RE.captures(line).unwrap();

    let min = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let max = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();

    return (String::from(&cap[3]), min, max, String::from(&cap[4]))

}

fn part1(input: &str) -> usize {
    let mut valid: usize = 0;
    for line in input.lines() {
        let (letter, min, max, text) = parse(&line.trim());
        let appearances: i64 = text.matches(&letter).count() as i64;
        if appearances >= min && appearances <= max {
            valid += 1;
        }
    }
    valid
}
fn part2(input: &str) -> usize {
    let mut valid: usize = 0;
    for line in input.lines() {
        let (letter, pos1, pos2, text) = parse(&line.trim());
        let pos1 = pos1 as usize;
        let pos2 = pos2 as usize;
        let found1 = &text[pos1-1..pos1] == letter;
        let found2 = &text[pos2-1..pos2] == letter;
        if found1 != found2 { // xor~
            valid += 1;
        }
    }
    valid
}
