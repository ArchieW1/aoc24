fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<Vec<i32>>  {
    input.lines()
         .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
         .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let factor = (report[1] - report[0]).signum();
    (1..report.len())
        .all(|i| {
            let change = factor * (report[i] - report[i - 1]);
            1 <= change && change <= 3
        })
}

fn part1(input: &str) -> i32 {
    let reports = parse_input(input);
    reports.iter().filter(|&r| is_safe(r)).count() as i32
}

fn part2(input: &str) -> i32 {
    let reports = parse_input(input);
    reports.iter().filter(|&r|
        is_safe(r) || (0..r.len()).any(|i| {
            let mut vec = r.to_vec();
            vec.remove(i);
            is_safe(&vec)
        }))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1_example() {
        let input = include_str!("example.txt");
        let output = part1(input);
        assert_eq!(output, 2);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("example.txt");
        let output = part2(input);
        assert_eq!(output, 4);
    }
}