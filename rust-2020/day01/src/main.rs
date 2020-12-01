#![allow(unused)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;

fn main() -> io::Result<()> {
    part1();
    part2()
}

fn part1() -> io::Result<()> {
    let fd = File::open("input/day01").unwrap();
    let reader = BufReader::new(fd);
    let mut numbers: Vec<i32> = vec![];
    for line in reader.lines() {
        let number = line.unwrap().parse::<i32>().unwrap();
        numbers.push(number);
    }
    'outer: for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers.iter().enumerate() {
            if j <= i { continue }
            if num1 + num2 == 2020 {
                println!("part1: {}", num1 * num2)
            }
        }
    }
    Ok(())
}
fn part2() -> io::Result<()> {
    let fd = File::open("input/day01").unwrap();
    let reader = BufReader::new(fd);
    let mut numbers: Vec<i32> = vec![];
    for line in reader.lines() {
        let number = line.unwrap().parse::<i32>().unwrap();
        numbers.push(number);
    }
    'x: for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers.iter().enumerate() {
            for (k, num3) in numbers.iter().enumerate() {
                if j <= i || k <= j { continue }
                if num1 + num2 + num3 == 2020 {
                    println!("part2: {}", num1 * num2 * num3)
                }
            }
        }
    }
    Ok(())
}
