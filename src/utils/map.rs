use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use super::{Direction, Pos2D};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Node {
    pub position: Pos2D,
    pub direction: Direction,
}

type Tile = usize;

#[derive(Debug)]
pub struct Map2D<T> {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<T>,
    pub start: Tile,
    pub end: Tile,
}

impl<T> Map2D<T>
where
    T: Display + PartialEq,
{
    pub fn pos2idx(&self, position: &Pos2D) -> Tile {
        position.to_idx(self.width, self.height).unwrap()
    }

    pub fn idx2pos(&self, idx: usize) -> Pos2D {
        Pos2D::from_idx(idx, self.width, self.height).unwrap()
    }

    pub fn dist(&self, a: Tile, b: Tile) -> i32 {
        let a_pos = self.idx2pos(a);
        let b_pos = self.idx2pos(b);
        (a_pos.dist(&b_pos) * 10.) as i32
    }

    pub fn get_neighbors(&self, node: Tile, neighbor_type: T, include_corners: bool) -> Vec<Tile> {
        self.idx2pos(node)
            .neighbors(include_corners)
            .iter()
            .filter(|pos| self.is_valid_pos(&pos))
            .map(|pos| self.pos2idx(pos))
            .filter(|&node| self.tiles[node] == neighbor_type)
            .collect()
    }

    pub fn is_valid_pos(&self, position: &Pos2D) -> bool {
        position.x >= 0
            && position.x < (self.width as i32)
            && position.y >= 0
            && position.y < (self.height as i32)
    }

    pub fn display(&self) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if idx % self.width == 0 {
                println!("");
            }
            print!("{}", tile);
        }
        println!("\n");
    }

    pub fn display_path(&self, path: &Vec<Tile>) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if idx % self.width == 0 {
                println!("");
            }

            if path.contains(&idx) {
                print!("@");
            } else {
                print!("{}", tile);
            }
        }
        println!("\n");
    }
}

#[derive(Eq, PartialEq)]
struct State {
    cost: i32,
    node: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStar {
    turn_penalty: i32,
}

impl AStar {
    pub fn new(turn_penalty: i32) -> Self {
        AStar { turn_penalty }
    }

    fn heuristic(&self, node: &Node, goal: &Node) -> i32 {
        let manhattan_distance =
            (node.position.x - goal.position.x).abs() + (node.position.y - goal.position.y).abs();
        let min_turns = self.estimate_min_turns(node, goal);
        manhattan_distance + min_turns * self.turn_penalty
    }

    fn estimate_min_turns(&self, node: &Node, goal: &Node) -> i32 {
        // Simplified estimation: 0 if same direction, 1 otherwise
        if node.direction == self.get_direction_to_goal(node, goal) {
            0
        } else {
            1
        }
    }

    fn get_direction_to_goal(&self, node: &Node, goal: &Node) -> Direction {
        if (goal.position.x - node.position.x).abs() > (goal.position.y - node.position.y).abs() {
            if goal.position.x > node.position.x {
                Direction::East
            } else {
                Direction::West
            }
        } else {
            if goal.position.y < node.position.y {
                Direction::North
            } else {
                Direction::South
            }
        }
    }

    fn get_neighbors<T: Clone + PartialEq>(
        &self,
        og_map: &Map2D<T>,
        node: &Node,
        neighbor_type: &T,
    ) -> Vec<Node> {
        let mut actual_neighbors = vec![];

        let neighbors = node.position.neighbors(false);
        for neighbor_pos in neighbors {
            let map_idx = neighbor_pos.to_idx(og_map.width, og_map.height).unwrap();
            let tile = og_map.tiles[map_idx].clone();
            if tile == *neighbor_type {
                let direction = node.position.get_direction(neighbor_pos);
                actual_neighbors.push(Node {
                    position: neighbor_pos,
                    direction,
                });
            }
        }

        actual_neighbors
    }

    fn calculate_cost(&self, current: &Node, next: &Node) -> i32 {
        let base_cost = 1;
        if current.direction != next.direction {
            base_cost + self.turn_penalty
        } else {
            base_cost
        }
    }

    pub fn find_path<T: Clone + PartialEq>(
        &self,
        start: Node,
        goal: Node,
        map: &Map2D<T>,
        neighbor_type: &T,
    ) -> Option<Vec<Node>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();

        open_set.push(State {
            cost: 0,
            node: start.clone(),
        });
        g_score.insert(start.clone(), 0);

        while let Some(State { node: current, .. }) = open_set.pop() {
            if current == goal {
                return Some(self.reconstruct_path(came_from, current));
            }

            for neighbor in self.get_neighbors(&map, &current, neighbor_type) {
                let tentative_g_score =
                    g_score[&current] + self.calculate_cost(&current, &neighbor);

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor.clone(), current.clone());
                    g_score.insert(neighbor.clone(), tentative_g_score);
                    let f_score = tentative_g_score + self.heuristic(&neighbor, &goal);
                    open_set.push(State {
                        cost: f_score,
                        node: neighbor,
                    });
                }
            }
        }

        None
    }

    fn reconstruct_path(&self, came_from: HashMap<Node, Node>, current: Node) -> Vec<Node> {
        let mut path = vec![current.clone()];
        let mut current = current;
        while let Some(prev) = came_from.get(&current) {
            path.push(prev.clone());
            current = prev.clone();
        }
        path.reverse();
        path
    }
}
