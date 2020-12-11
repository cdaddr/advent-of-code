use std::hint::unreachable_unchecked;

fn main() {
    let input = std::fs::read_to_string("input/day09").unwrap();

    let ans = part1(&input, 25);
    println!("part 1: {}", ans);

    let ans = part2(&input, ans);
    println!("part 2: {}", ans);
}

fn sum_of_pair(num: i64, others: &[i64]) -> bool {
    for (idx, other1) in others.iter().enumerate() {
        for other2 in others[idx..].iter() {
            if other1 + other2 == num {
                return true
            }
        }
    }
    false
}

fn part1(input: &str, preamble_size: usize) -> i64 {
    let nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut missing = nums
        .iter()
        .enumerate()
        .filter(|(idx, &num)|
            *idx > preamble_size
                && !sum_of_pair(num, &nums[*idx-preamble_size..=*idx]))
        .map(|x| *x.1)
        ;
    match &missing.next() {
        None => { unreachable!()}
        Some(x) => {return *x}
    }
}

fn part2(input: &str, part1_ans: i64) -> i64 {
    let nums: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    'outer: for (idx, &num) in nums.iter().enumerate() {
        let mut idx2 = idx;
        let mut sum = 0;
        loop {
            sum += nums[idx2];
            if sum == part1_ans {
                let range = &nums[idx..=idx2];
                return range.iter().min().unwrap() + range.iter().max().unwrap()

            }
            if sum > part1_ans {
                continue 'outer;
            }
            idx2 += 1;
        }
    }
    unreachable!()
}

#[test]
fn test() {
    let input = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let part1_ans = part1(input, 5);
    assert_eq!(part1_ans, 127);
    assert_eq!(part2(input, part1_ans), 62);
}

