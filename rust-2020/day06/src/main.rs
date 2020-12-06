use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

fn main() {
    let input = std::fs::read_to_string("input/day06").unwrap();

    let ans = part1(&input);
    println!("part 1: {}", ans);

    let ans = part2(&input);
    println!("part 2: {}", ans);
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    for group in input.split("\n\n") {
        let letters: Vec<char> = group
            .replace(char::is_whitespace, "")
            .chars()
            .collect()
            ;
        count += HashSet::<char>::from_iter(letters).len();
    }
    count
}

// Borrow checker kicked the crap out of me,
// and my code got longer and longer the more I struggled.
//
// How do I rust?
fn part2(input: &str) -> usize {
    let mut count = 0;
    for group in input.split("\n\n") {
        let mut found: HashMap<char, usize> = HashMap::new();
        let people = group.trim().split("\n").collect::<Vec<_>>();
        let max = people.len();
        for person in people {
            let letters: Vec<char> = person
                .replace(char::is_whitespace, "")
                .chars()
                .collect()
                ;
            for letter in letters {
                match found.get(&letter) {
                    None => {found.insert(letter, 1);}
                    Some(_) => {found.insert(letter, found[&letter] + 1);}
                }
            }
        }
        count += found.iter().filter(|(_, &v)| v == max).collect::<Vec<_>>().len();
    }
    count
}

#[test]
fn test() {
    let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b
";
    assert_eq!(part1(input), 11);
    assert_eq!(part2(input), 6);
}

