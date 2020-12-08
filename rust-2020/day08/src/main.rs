#![feature(str_split_once)]

use std::collections::HashSet;
use std::hint::unreachable_unchecked;

fn main() {
    let input = std::fs::read_to_string("input/day08").unwrap();

    let ans = part1(&input);
    println!("part 1: {}", ans);

    let ans = part2(&input);
    println!("part 2: {}", ans);
}

fn part1(input: &str) -> i64 {
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut seen: HashSet<i64> = HashSet::new();


    loop {
        if seen.get(&pc).is_some() {
            return acc;
        }
        seen.insert(pc);
        let curr = lines[pc as usize];
        let (op, arg) = curr.split_once(char::is_whitespace).unwrap();
        match op {
            "nop" => { pc += 1; }
            "acc" => {
                acc += arg.parse::<i64>().unwrap();
                pc += 1;
            }
            "jmp" => { pc += arg.parse::<i64>().unwrap(); }
            _ => unreachable!()
        }
    }
}

fn part2(input: &str) -> i64 {
    let lines: Vec<String> = input.trim().lines().map(String::from).collect();
    let max = lines.len() as i64;

    let mut curr: usize = 0;
    'outer: loop {
        if curr >= max as usize {
            return 0;
        }
        let mut lines = lines.clone();
        let line = lines.get(curr).unwrap();
        let (op, _) = line.split_once(char::is_whitespace).unwrap();
        match op {
            "nop" => { lines[curr as usize] = line.replace("nop", "jmp"); }
            "jmp" => { lines[curr as usize] = line.replace("jmp", "nop"); }
            _ => { curr += 1; continue 'outer; }
        }
        let mut seen: HashSet<i64> = HashSet::new();
        let mut pc: i64 = 0;
        let mut acc: i64 = 0;
        loop {
            if pc == max {
                return acc;
            }
            if seen.get(&pc).is_some() {
                curr += 1;
                continue 'outer;
            }
            seen.insert(pc);
            let curr = lines.get(pc as usize).unwrap();
            let (op, arg) = curr.split_once(char::is_whitespace).unwrap();
            match op {
                "nop" => { pc += 1; }
                "acc" => {
                    acc += arg.parse::<i64>().unwrap();
                    pc += 1;
                }
                "jmp" => { pc += arg.parse::<i64>().unwrap(); }
                _ => unreachable!()
            }
        }
    }
}

#[test]
fn test() {
    let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), 8);
}

