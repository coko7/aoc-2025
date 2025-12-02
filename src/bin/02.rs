use anyhow::{Context, Result};

advent_of_code::solution!(2);

fn parse_id_range(range: &str) -> Result<(u64, u64)> {
    let (start, end) = range
        .split_once('-')
        .context("should be a dash in middle")?;

    let start = start.parse::<u64>()?;
    let end = end.parse::<u64>()?;
    Ok((start, end))
}

fn has_odd_digits(n: u64) -> bool {
    if n == 0 {
        return true;
    }
    let digits = n.ilog10() as usize + 1;
    (digits & 1) != 0
}

fn is_rep_digit(n: u64) -> bool {
    let mut prev = None;
    let mut num = n;
    while num != 0 {
        let digit = num % 10;
        num /= 10;
        if let Some(prev) = prev {
            if digit != prev {
                return false;
            }
        }
        prev = Some(digit);
    }
    true
}

fn find_all_divisors(n: u64) -> Vec<u64> {
    (1..n).filter(|&d| n % d == 0).collect()
}

fn are_all_patterns_equal(patterns: &[&str]) -> bool {
    patterns.windows(2).all(|p| p[0] == p[1])
}

fn get_all_subpatterns(val: &str, pattern_len: usize) -> Vec<&str> {
    let num_patterns = val.len() / pattern_len;
    (0..num_patterns)
        .map(|idx| &val[idx * pattern_len..(idx * pattern_len + pattern_len)])
        .collect::<Vec<_>>()
}

fn is_symmetric(n: u64) -> bool {
    let str_val = n.to_string();
    let mid_point = str_val.len() / 2;

    let left = &str_val[0..mid_point];
    let right = &str_val[mid_point..];

    left == right
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for id_range in input.trim().split(',') {
        let (start, end) = parse_id_range(id_range).unwrap();
        for id in start..=end {
            if has_odd_digits(id) {
                continue;
            }

            if is_symmetric(id) {
                sum += id;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for id_range in input.trim().split(',') {
        let (start, end) = parse_id_range(id_range).unwrap();
        for id in start..=end {
            let str_id = id.to_string();
            let possible_pattern_lengths = find_all_divisors(str_id.len() as u64)
                .iter()
                .map(|&l| l as usize)
                .collect::<Vec<usize>>();

            for pattern_len in possible_pattern_lengths {
                let all_patterns = get_all_subpatterns(&str_id, pattern_len);
                if are_all_patterns_equal(&all_patterns) {
                    sum += id;
                    break;
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
