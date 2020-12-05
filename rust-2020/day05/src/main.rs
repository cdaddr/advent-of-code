fn main() {
    let input = std::fs::read_to_string("input/day05").unwrap();

    let ans = part1(&input);
    println!("part 1: {}", ans);

    let ans = part2(&input);
    println!("part 2: {}", ans);
}

fn part1(input: &str) -> usize {
    seat_ids(input).max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut all_seats = seat_ids(input).collect::<Vec<_>>();
    all_seats.sort();

    for (i, seat) in all_seats.iter().enumerate() {
        if all_seats[i+1] != seat + 1 {
            return seat + 1;
        }
    }
    unreachable!()
}

fn seat_ids(input: &str) -> impl Iterator<Item=usize> + '_ {
    input
        .lines()
        .map(seat_id)
}

fn seat_id(pattern: &str) -> usize {
    let num = pattern
        .replace('F', "0")
        .replace('B', "1")
        .replace('R', "1")
        .replace('L', "0");
    let num = usize::from_str_radix(&num, 2).unwrap();
    num
}

#[test]
fn test() {
    assert_eq!(seat_id("FBFBBFFRLR"), 357);
    assert_eq!(seat_id("BFFFBBFRRR"), 567);
    assert_eq!(seat_id("FFFBBBFRRR"), 119);
    assert_eq!(seat_id("BBFFBBFRLL"), 820);
}

