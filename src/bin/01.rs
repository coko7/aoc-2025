use anyhow::{Context, Result, bail};

advent_of_code::solution!(1);

fn parse_rotation(value: &str) -> Result<(i32, i32)> {
    let dir = value.chars().next().context("expects a char")?;
    let sign = match dir {
        'L' => -1,
        'R' => 1,
        _ => bail!("invalid rotation char"),
    };
    let amount = value[1..].parse::<i32>()?;
    Ok((sign, amount))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut zeroed = 0;
    let mut pos = 50;

    for instr in input.lines() {
        let (sign, amount) = parse_rotation(instr).unwrap();
        pos += sign * amount;
        if pos % 100 == 0 {
            zeroed += 1;
        }
    }

    Some(zeroed)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut zeroed = 0;
    let mut pos = 50;

    for instr in input.lines() {
        let (sign, amount) = parse_rotation(instr).unwrap();
        for _ in 0..amount {
            pos += sign;
            if pos % 100 == 0 {
                zeroed += 1;
            }
        }
    }

    Some(zeroed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
