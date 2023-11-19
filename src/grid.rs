use anyhow::Result;
use itertools::Itertools;
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

impl From<MoveDirection> for (i32, i32) {
    fn from(value: MoveDirection) -> Self {
        use MoveDirection::*;
        match value {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
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

    pub fn move_tiles(&mut self, direction: MoveDirection) {
        // let (i_sign, j_sign) = direction.into();

        use MoveDirection::*;
        match direction {
            Up => todo!(),
            Down => {
                for j in 0..N {
                    let mut border = N - 1;
                    for i in (0..N).rev() {
                        if self.tiles[i][j] != 0 {
                            while self.tiles[border][j] != 0
                                && self.tiles[border][j] != self.tiles[i][j]
                            {
                                border -= 1;
                            }

                            if border != i {
                                self.tiles[border][j] += self.tiles[i][j];
                                self.tiles[i][j] = 0;
                            }
                        }
                    }
                }
            }
            Left => todo!(),
            Right => {
                for i in 0..N {
                    let mut border = N - 1;
                    for j in (0..N).rev() {
                        if self.tiles[i][j] != 0 {
                            while self.tiles[i][border] != 0
                                && self.tiles[i][border] != self.tiles[i][j]
                            {
                                border -= 1;
                            }

                            if border != j {
                                self.tiles[i][border] += self.tiles[i][j];
                                self.tiles[i][j] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn is_full(&self) -> bool {
        self.tiles.iter().flatten().all(|tile| *tile == 0)
    }

    pub fn can_move(&self, direction: MoveDirection) -> bool {
        use MoveDirection::*;
        match direction {
            Up => todo!(),
            Down => (0..N)
                .map(|j| (0..N).map(move |i| self.tiles[i][j]))
                .any(|column| {
                    column.tuple_windows().any(|pair: (u32, u32)| {
                        (pair.0 != 0 && pair.1 == 0) || (pair.0 != 0 && pair.0 == pair.1)
                    })
                }),
            Left => todo!(),
            Right => todo!(),
        }
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

    macro_rules! test_default {
        ($($n: expr),*) => {
            $(assert_eq!(
                Grid::<$n>::default(),
                Grid {
                    tiles: [[0; $n]; $n],
                    score: 0
                }
            );
        )*
        };
    }

    #[test]
    fn default() {
        test_default!(3, 4, 5, 6, 7, 8, 9, 10);
    }

    #[test]
    fn move_tiles_down() {
        let mut grid = Grid::<4> {
            tiles: [[2, 0, 0, 2], [0, 2, 2, 4], [0, 0, 2, 8], [0, 0, 0, 4]],
            ..Default::default()
        };

        grid.move_tiles(MoveDirection::Down);
        assert_eq!(
            grid,
            Grid::<4> {
                tiles: [[0, 0, 0, 2], [0, 0, 0, 4], [0, 0, 0, 8], [2, 2, 4, 4]],
                ..Default::default()
            }
        );
    }

    #[test]
    fn move_tiles_right() {
        let mut grid = Grid::<4> {
            tiles: [[2, 0, 0, 0], [0, 2, 2, 0], [2, 4, 8, 16], [16, 8, 4, 2]],
            ..Default::default()
        };

        grid.move_tiles(MoveDirection::Right);
        assert_eq!(
            grid,
            Grid::<4> {
                tiles: [[0, 0, 0, 2], [0, 0, 0, 4], [2, 4, 8, 16], [16, 8, 4, 2]],
                ..Default::default()
            }
        );
    }

    #[test]
    fn can_move_down() {
        assert!(!Grid {
            tiles: [[0, 0], [0, 0]],
            ..Default::default()
        }
        .can_move(MoveDirection::Down));

        assert!(Grid {
            tiles: [[1, 0], [0, 0]],
            ..Default::default()
        }
        .can_move(MoveDirection::Down));

        assert!(Grid {
            tiles: [[0, 0, 0], [0, 0, 2], [0, 0, 2]],
            ..Default::default()
        }
        .can_move(MoveDirection::Down));

        assert!(!Grid {
            tiles: [[0, 2, 0], [0, 4, 0], [0, 8, 0]],
            ..Default::default()
        }
        .can_move(MoveDirection::Down));
    }
}
