#![allow(unused)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;

fn main() {
    let input = std::fs::read_to_string("input/day01").unwrap();
    let numbers = lines_to_numbers(&input);
    if let Some(ans) = part1(&numbers) {
        println!("part1: {}", ans);
    }
    if let Some(ans) = part2(&numbers) {
        println!("part2: {}", ans);
    }
}

fn lines_to_numbers(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn part1(numbers: &Vec<i64>) -> Option<i64> {
    'outer: for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers.iter().enumerate() {
            if j <= i { continue }
            if num1 + num2 == 2020 {
                return Some(num1 * num2)
            }
        }
    }
    return None
}
fn part2(numbers: &Vec<i64>) -> Option<i64> {
    let fd = File::open("input/day01").unwrap();
    let reader = BufReader::new(fd);
    let mut numbers: Vec<i64> = vec![];
    for line in reader.lines() {
        let number = line.unwrap().parse::<i64>().unwrap();
        numbers.push(number);
    }
    'x: for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers.iter().enumerate() {
            for (k, num3) in numbers.iter().enumerate() {
                if j <= i || k <= j { continue }
                if num1 + num2 + num3 == 2020 {
                    return Some(num1 * num2 * num3)
                }
            }
        }
    }
    return None
}

#[test]
fn test_day01() {
    let test_input = "1721\n979\n366\n299\n675\n1456\n";
    let numbers = lines_to_numbers(&test_input);
    assert_eq!(part1(&numbers).unwrap(), 514579);
}