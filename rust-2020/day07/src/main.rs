#![feature(str_split_once)]

use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input/day07").unwrap();

    let ans = part1(&input);
    println!("part 1: {}", ans);

    let ans = part2(&input);
    println!("part 2: {}", ans);
}

fn bag_name(input: &str) -> &str {
    Regex::new(r"^(\w+ \w+)").unwrap().captures(input).unwrap().get(1).unwrap().as_str()
}

fn parse_bag(input: &str) -> (&str, Vec<&str>) {
    let caps = Regex::new(
        r"^(\w+ \w+) bags contain (no other bags|(\d+ \w+ \w+ bags?)(, \d+ \w+ \w+ bags?)*)\."
    ).unwrap().captures(input).unwrap();
    let subs = caps.get(2).unwrap().as_str();
    return (caps.get(1).unwrap().as_str(),
            if subs == "no other bags" { vec![] } else { subs.split(", ").collect()}
            )
}

fn part1(input: &str) -> usize {
    let bags = bags_containing_gold(input);
    bags.len()
}

fn bags_containing_gold(input: &str) -> HashSet<&str> {
    let mut stack: Vec<&str> = Vec::new();
    let mut found: HashSet<&str> = HashSet::new();
    stack.push("shiny gold");
    let lines = input.trim().lines().collect::<Vec<_>>();

    loop {
        match stack.pop() {
            None => {
                return found;
            }
            Some(bag) => {
                for line in lines.iter() {
                    if found.contains(line) { continue }
                    if line.contains(bag) && ! line.starts_with(bag) {
                        found.insert(line);
                        stack.push(bag_name(line));
                    }
                }
            }
        }
    }

}

fn parse_sub(sub: &str) -> (usize, String) {
    let parts = sub.split_whitespace().collect::<Vec<_>>();
    let n = parts[0].parse::<usize>().unwrap();
    let name = format!("{} {}", parts[1], parts[2]);
    (n, name)

}

fn bags_inside(bags: &HashMap<&str, Vec<(usize, String)>>, bag: &str) -> usize {
    let mut count = 0;
    for (n, sub) in bags.get(bag).unwrap() {
        count += n + (n * bags_inside(&bags, sub));
    }
    count
}

fn part2(input: &str) -> usize {
    let mut bags: HashMap<&str, Vec<(usize, String)>> = HashMap::new();
    for line in input.trim().lines() {
        let (name, subs) = parse_bag(line);
        let subs = subs.iter().map(|sub| parse_sub(sub)).collect();
        bags.insert(name, subs);
    }

    bags_inside(&bags, "shiny gold")
}

#[test]
fn test() {
    let input = "\
    light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    assert_eq!(part1(input), 4);
    assert_eq!(part2(input), 32);
    let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    assert_eq!(part2(input), 126);
}

