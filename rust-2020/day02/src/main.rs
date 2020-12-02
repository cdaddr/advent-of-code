#[macro_use]
extern crate lazy_static;

use regex::Regex;
use anyhow::Result;

fn main() {
    let input = std::fs::read_to_string("input/day02").unwrap();
    let ans = part1(&input);
    println!("part1: {}", ans);

    let ans = part2(&input);
    println!("part2: {}", ans)
}

struct Password {
    min: i64,
    max: i64,
    letter: String,
    text: String,
}

fn parse(line: &str) -> Result<Password> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (\w+)$").unwrap();
    }

    let cap: regex::Captures = RE.captures(line).unwrap();

    return Ok(Password {
        min: cap[1].parse::<i64>()?,
        max: cap[2].parse::<i64>()?,
        letter: String::from(&cap[3]),
        text: String::from(&cap[4]),
    });

}

fn part1(input: &str) -> usize {
    let mut valid: usize = 0;
    for line in input.lines() {
        if let Ok(pw) = parse(&line.trim()) {
            let appearances: i64 = pw.text.matches(&pw.letter).count() as i64;
            if appearances >= pw.min && appearances <= pw.max {
                valid += 1;
            }
        }
    }
    valid
}

fn part2(input: &str) -> usize {
    let mut valid: usize = 0;
    for line in input.lines() {
        if let Ok(pw) = parse(&line.trim()) {
            let pos1 = pw.min as usize;
            let pos2 = pw.max as usize;
            let found1 = &pw.text[pos1 - 1..pos1] == pw.letter;
            let found2 = &pw.text[pos2 - 1..pos2] == pw.letter;
            if found1 != found2 { // xor~
                valid += 1;
            }
        }
    }
    valid
}
