use super::Direction;
use anyhow::Result;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Pos2D {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub enum Pos2DError {
    OutOfBounds(String),
}

impl Pos2D {
    pub fn new(x: i32, y: i32) -> Pos2D {
        Pos2D { x, y }
    }

    pub fn from_idx(idx: usize, width: usize, height: usize) -> Result<Pos2D, Pos2DError> {
        let x = (idx % width) as i32;
        let y = (idx / width) as i32;

        if x < 0 || x >= (width as i32) || y < 0 || y >= (height as i32) {
            let msg = format!("Idx out of bounds: {} => (x: {}, y: {})", idx, x, y);
            return Err(Pos2DError::OutOfBounds(msg));
        }

        Ok(Pos2D::new(x, y))
    }

    pub fn to_idx(&self, width: usize, height: usize) -> Result<usize, Pos2DError> {
        if self.x < 0 || self.x >= (width as i32) || self.y < 0 || self.y >= (height as i32) {
            let msg = format!("Pos out of bounds: (x: {}, y: {})", self.x, self.y);
            return Err(Pos2DError::OutOfBounds(msg));
        }

        let idx = self.y as usize * width + self.x as usize;
        Ok(idx)
    }

    pub fn right(&self, offset: i32) -> Pos2D {
        Pos2D::new(self.x + offset, self.y)
    }

    pub fn left(&self, offset: i32) -> Pos2D {
        Pos2D::new(self.x - offset, self.y)
    }

    pub fn up(&self, offset: i32) -> Pos2D {
        Pos2D::new(self.x, self.y - offset)
    }

    pub fn down(&self, offset: i32) -> Pos2D {
        Pos2D::new(self.x, self.y + offset)
    }

    pub fn get_direction(&self, next: Pos2D) -> Direction {
        if self.x == next.x {
            if next.y < self.y {
                Direction::North
            } else {
                Direction::South
            }
        } else {
            if next.x > self.x {
                Direction::East
            } else {
                Direction::West
            }
        }
    }

    pub fn neighbors(&self, include_corners: bool) -> Vec<Pos2D> {
        let mut neighbors = vec![];
        for y in self.y - 1..=self.y + 1 {
            for x in self.x - 1..=self.x + 1 {
                if !include_corners && y != self.y && x != self.x {
                    continue;
                }

                // skip self
                if x == self.x && y == self.y {
                    continue;
                }

                neighbors.push(Pos2D::new(x, y));
            }
        }
        neighbors
    }

    pub fn dist(&self, other: &Pos2D) -> f64 {
        let xd = (other.x - self.x).pow(2) as f64;
        let yd = (other.y - self.y).pow(2) as f64;
        (xd + yd).sqrt()
    }

    pub fn add<T>(&self, other: T) -> Pos2D
    where
        T: AsRef<Pos2D>,
    {
        let other = other.as_ref();
        Pos2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
