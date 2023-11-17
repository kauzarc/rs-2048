use anyhow::Result;
use rand::{prelude::Distribution, seq::IteratorRandom};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<const N: usize> {
    tiles: [[u32; N]; N],
    score: u32,
}

#[derive(Debug, Error)]
pub enum GridError {
    #[error("The grid is full")]
    GridFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl<const N: usize> Grid<N> {
    pub fn random_spawn_tile(&mut self) -> Result<()> {
        let mut rng = rand::thread_rng();

        let tile = self
            .tiles
            .iter_mut()
            .flatten()
            .filter(|tile| **tile == 0)
            .choose(&mut rng)
            .ok_or(GridError::GridFull)?;

        let dist = rand::distributions::Bernoulli::new(0.75)?;
        *tile = if dist.sample(&mut rng) { 2 } else { 4 };

        Ok(())
    }

    pub fn move_tiles(&mut self, direction: MoveDirection) -> Result<()> {
        use MoveDirection::*;

        match direction {
            Up => todo!(),
            Down => todo!(),
            Left => todo!(),
            Right => {
                for row in self.tiles.iter_mut() {
                    for i in (0..N).rev() {
                        if row[i] != 0 {
                            for j in (i + 1)..N {
                                if row[j] == row[i] {
                                    row[i] = 0;
                                    row[j] *= 2;
                                } else if row[j] != 0 {
                                    row[j - 1] = row[i];
                                    row[i] = 0;
                                    break;
                                } else if j == N - 1 {
                                    row[j] = row[i];
                                    row[i] = 0;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.tiles.iter().flatten().all(|tile| *tile == 0)
    }

    pub fn tiles(&self) -> &[[u32; N]; N] {
        &self.tiles
    }

    pub fn score(&self) -> &u32 {
        &self.score
    }
}

impl<const N: usize> Default for Grid<N> {
    fn default() -> Self {
        Self {
            tiles: [[Default::default(); N]; N],
            score: Default::default(),
        }
    }
}

impl<const N: usize> fmt::Display for Grid<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "grid:")?;
        for row in self.tiles {
            writeln!(f, "|{}|", row.map(|tile| format!("{:^5}", tile)).join("|"))?;
        }

        write!(f, "score: {}", self.score)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            Grid::<4>::default(),
            Grid {
                tiles: [[0; 4]; 4],
                score: 0
            }
        );
    }
}
