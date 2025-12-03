advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut output_joltage = 0;
    for line in input.lines() {
        let mut batteries = line.chars();
        let mut max_bank = 0;
        let mut last_bat = batteries.next().unwrap().to_digit(10).unwrap() as u64;

        for bat in batteries {
            let bat = bat.to_digit(10).unwrap() as u64;
            let bank = 10 * last_bat + bat;
            if bank > max_bank {
                max_bank = bank;
            }
            if bat > last_bat {
                last_bat = bat;
            }
        }
        output_joltage += max_bank;
    }
    Some(output_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
