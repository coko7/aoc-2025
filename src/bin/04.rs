use std::fmt::Display;

use advent_of_code::utils::{Map2D, Pos2D};
use anyhow::Result;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileType {
    PaperRoll,
    Empty,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::PaperRoll => write!(f, "{}", "@"),
            TileType::Empty => write!(f, "{}", "."),
        }
    }
}

#[derive(Debug)]
struct Map {
    pub base: Map2D<TileType>,
}

fn from_input(input: &str) -> Result<Map> {
    let height = input.lines().count();
    let width = input.lines().last().unwrap().chars().count();

    let mut tiles = vec![TileType::Empty; height * width];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = Pos2D::new(x as i32, y as i32);

            let tile = match c {
                '@' => TileType::PaperRoll,
                '.' => TileType::Empty,
                _ => panic!("Unknown tile: {}", c),
            };

            let idx = position.to_idx(width, height).unwrap();
            tiles[idx] = tile;
        }
    }

    Ok(Map {
        base: Map2D {
            width,
            height,
            tiles,
            start: 0,
            end: 0,
        },
    })
}

fn get_ccessible_paper_rolls(map: &Map) -> Vec<usize> {
    map.base
        .tiles
        .iter()
        .enumerate()
        .filter(|&(_idx, &tile)| tile == TileType::PaperRoll)
        .filter(|&(idx, _tile)| map.base.get_neighbors(idx, TileType::PaperRoll, true).len() < 4)
        .map(|(idx, _tile)| idx)
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = from_input(input).unwrap();
    let mut accessible_paper_rolls = 0;

    for (idx, &tile) in map.base.tiles.iter().enumerate() {
        if tile != TileType::PaperRoll {
            continue;
        }

        let neighbors = map.base.get_neighbors(idx, TileType::PaperRoll, true);
        if neighbors.len() < 4 {
            accessible_paper_rolls += 1;
        }
    }

    Some(accessible_paper_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = from_input(input).unwrap();
    let mut removed_rps = 0;
    loop {
        let accessible_prs = get_ccessible_paper_rolls(&map);
        if accessible_prs.is_empty() {
            break;
        }

        for pr_idx in accessible_prs {
            map.base.tiles[pr_idx] = TileType::Empty;
            removed_rps += 1;
        }
    }
    Some(removed_rps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
