fn main() {
    let input = std::fs::read_to_string("input/day03").unwrap();

    let ans = part1(&input, Slope{right:3,down:1});
    println!("part 1: {}", ans);

    let ans = part2(&input);
    println!("part 2: {}", ans);
}

struct Slope {
    right: usize,
    down: usize,
}

fn part1(input: &str, slope: Slope ) -> usize {
    let parsed: Vec<_> = input
        .lines()
        .map(|line| line.match_indices(r"#"))
        .collect();
    let width = input.lines().into_iter().next().unwrap().len();
    let mut trees = 0usize;
    let mut pos = 0usize;
    let mut curr = 0;
    loop {
        let line = &parsed[curr];
        let line_trees: Vec<_> = line.clone().map(|idx| idx.0).collect();
        if line_trees.contains(&pos) {
            trees += 1;
        }

        pos = (pos + slope.right) % width;
        curr += slope.down;
        if curr >= parsed.len() {
            break;
        }
    }

    trees
}

fn part2(input: &str) -> usize {
    let slopes = vec![
        Slope{right: 1, down: 1},
        Slope{right: 3, down: 1},
        Slope{right: 5, down: 1},
        Slope{right: 7, down: 1},
        Slope{right: 1, down: 2},
    ];
    let mut ans = 1;
    for slope in slopes {
        ans = ans * part1(input, slope);
    }
    ans
}

#[test]
fn test_day3() {
    let str = "..##.......\n\
                     #...#...#..\n\
                     .#....#..#.\n\
                     ..#.#...#.#\n\
                     .#...##..#.\n\
                     ..#.##.....\n\
                     .#.#.#....#\n\
                     .#........#\n\
                     #.##...#...\n\
                     #...##....#\n\
                     .#..#...#.#\n";
    assert_eq!(7, part1(str, Slope{right: 3, down: 1}));
    assert_eq!(336, part2(str));
}