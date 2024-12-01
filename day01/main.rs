use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines().map(|l| {
        let mut pair = l.split_whitespace();
        (
            pair.next().unwrap().parse::<i32>().unwrap(),
            pair.next().unwrap().parse::<i32>().unwrap(),
        )
    }).unzip()
}

fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

fn part2(input: &str) -> i32 {
    let (left,  right) = parse_input(input);
    let right_map = right.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    left.iter().fold(0, |acc, x| {
        acc + right_map.get(x).unwrap_or(&0) * x
    }).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1_example() {
        let input = include_str!("example.txt");
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("example.txt");
        assert_eq!(part2(input), 31);
    }
}