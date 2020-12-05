use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input/day04").unwrap();

    let ans = part1(&input);
    println!("part 1: {}", ans);

    let ans = part2(&input);
    println!("part 2: {}", ans);
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .filter(|&str| validate1(str))
        .count()
}

fn validate1(input: &str) -> bool {
    // duh, cheating
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required.iter().all(|field| input.contains(field))
}

fn valid_year(val: &str, min: usize, max: usize) -> bool {
    (min..=max).contains(&val.parse::<usize>().unwrap())
}

fn valid_height(val: &str) -> bool {
    let cm = Regex::new(r"(\d+)cm").unwrap();
    let inches = Regex::new(r"(\d+)in").unwrap();
    if let Some(mat) = cm.captures(val) {
        (150..=193).contains(&mat[1].parse::<usize>().unwrap())
    } else if let Some(mat) = inches.captures(val) {
        (59..=76).contains(&mat[1].parse::<usize>().unwrap())
    } else {
        false
    }
}

fn valid_hexcolor(val: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(val)
}

fn valid_eyecolor(val: &str) -> bool {
    let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    re.is_match(val)
}

fn valid_pid(val: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(val)
}

// validate1 already tells us if all fields are present; validate2 checks the values for each key
fn validate2(input: &str) -> bool {
    if ! validate1(input) {
        return false
    }
    for field in input.trim().split(char::is_whitespace) {
        let field: Vec<_> = field.split(':').collect();
        if ! match field[0] {
            "byr" => valid_year(field[1], 1920, 2002),
            "iyr" => valid_year(field[1], 2010, 2020),
            "eyr" => valid_year(field[1], 2020, 2030),
            "hgt" => valid_height(field[1]),
            "hcl" => valid_hexcolor(field[1]),
            "ecl" => valid_eyecolor(field[1]),
            "pid" => valid_pid(field[1]),
            "cid" => true,
            _ => false
        } {
            println!("{:?}", field);
            return false
        }

    }
    true
}


fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|&str| validate2(str))
        .count()
}

