use core::panic;

advent_of_code::solution!(1);

fn parse_rotation(value: &str) -> i32 {
    let dir = value.chars().next().unwrap();
    let mul = match dir {
        'L' => -1,
        'R' => 1,
        _ => panic!("should not happen"),
    };
    let amount = value.strip_prefix(dir).unwrap().parse::<i32>().unwrap();
    mul * amount
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut zeroed = 0;
    let mut pos = 50;
    for instr in input.lines() {
        let rot = parse_rotation(instr);
        pos = (pos + rot).rem_euclid(100);
        if pos == 0 {
            zeroed += 1;
        }
    }
    Some(zeroed)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut zeroed = 0;
    let mut pos = 50;
    for instr in input.lines() {
        let rot = parse_rotation(instr);
        for _ in 0..rot.abs() {
            pos += rot.signum();
            pos = pos.rem_euclid(100);
            if pos == 0 {
                zeroed += 1;
            }
        }
        println!("{instr} => {pos} {zeroed}");
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
