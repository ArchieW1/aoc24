use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<(i32, i32)>  {
    let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    reg.captures_iter(input)
        .map(|c| {
            let a = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            (a, b)
        }).collect()
}

fn part1(input: &str) -> i32 {
    let numbers = parse_input(input);
    numbers.iter().fold(0, |acc, (a, b)| acc + a * b)
}

fn parse_input_dos(input: &str) -> Vec<(i32, i32)> {
    let reg = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();

    let mut is_do = true;
    reg.find_iter(input)
        .filter_map(|m| {
            let s = m.as_str();
            if s == "do()" {
                is_do = true;
                None
            } else if s == "don't()" {
                is_do = false;
                None
            } else if is_do {
                let parts: Vec<&str> = s.strip_prefix("mul(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(',')
                    .collect();
                let a = parts[0].parse::<i32>().unwrap();
                let b = parts[1].parse::<i32>().unwrap();
                Some((a, b))
            } else {
                None
            }
        }).collect()
}

fn part2(input: &str) -> i32 {
    let numbers = parse_input_dos(input);
    numbers.iter().fold(0, |acc, (a, b)| acc + a * b)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1_example() {
        let input = include_str!("example.txt");
        let output = part1(input);
        assert_eq!(output, 161);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("example2.txt");
        let output = part2(input);
        assert_eq!(output, 48);
    }
}