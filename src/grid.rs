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

impl MoveDirection {
    pub fn iterator() -> impl Iterator<Item = Self> {
        use MoveDirection::*;
        [Up, Down, Left, Right].iter().copied()
    }
}

pub fn grid_coord_iterator<const N: usize>() -> impl Iterator<Item = (usize, usize)> {
    (0..N).map(|i| (0..N).map(move |j| (i, j))).flatten()
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
        use MoveDirection::*;
        match direction {
            Up => {
                for j in 0..N {
                    let mut border = 0;
                    for i in 0..N {
                        if self.tiles[i][j] != 0 {
                            while self.tiles[border][j] != 0
                                && self.tiles[border][j] != self.tiles[i][j]
                            {
                                border += 1;
                            }

                            if border != i {
                                if self.tiles[border][j] == self.tiles[i][j] {
                                    self.score += 2 * self.tiles[i][j];
                                }

                                self.tiles[border][j] += self.tiles[i][j];
                                self.tiles[i][j] = 0;
                            }
                        }
                    }
                }
            }
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
                                if self.tiles[border][j] == self.tiles[i][j] {
                                    self.score += 2 * self.tiles[i][j];
                                }

                                self.tiles[border][j] += self.tiles[i][j];
                                self.tiles[i][j] = 0;
                            }
                        }
                    }
                }
            }
            Left => {
                for i in 0..N {
                    let mut border = 0;
                    for j in 0..N {
                        if self.tiles[i][j] != 0 {
                            while self.tiles[i][border] != 0
                                && self.tiles[i][border] != self.tiles[i][j]
                            {
                                border += 1;
                            }

                            if border != j {
                                if self.tiles[i][border] == self.tiles[i][j] {
                                    self.score += 2 * self.tiles[i][j];
                                }

                                self.tiles[i][border] += self.tiles[i][j];
                                self.tiles[i][j] = 0;
                            }
                        }
                    }
                }
            }
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
                                if self.tiles[i][border] == self.tiles[i][j] {
                                    self.score += 2 * self.tiles[i][j];
                                }

                                self.tiles[i][border] += self.tiles[i][j];
                                self.tiles[i][j] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn rotate_90_deg(&mut self, n_rotation: i32) {
        let tiles = self.tiles;

        match n_rotation.rem_euclid(4) {
            0 => (),
            1 => {
                for (i, j) in grid_coord_iterator::<N>() {
                    self.tiles[i][j] = tiles[j][N - i - 1];
                }
            }
            2 => {
                for (i, j) in grid_coord_iterator::<N>() {
                    self.tiles[i][j] = tiles[N - i - 1][N - j - 1];
                }
            }
            3 => {
                for (i, j) in grid_coord_iterator::<N>() {
                    self.tiles[i][j] = tiles[N - j - 1][i];
                }
            }

            _ => panic!("unreachable"),
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

    pub fn game_over(&self) -> bool {
        MoveDirection::iterator().all(|direction| !self.can_move(direction))
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
    fn random_spawn_tile() -> anyhow::Result<()> {
        let mut grid = Grid::<4>::default();
        grid.random_spawn_tile()?;

        assert_eq!(
            grid.tiles
                .iter()
                .flatten()
                .filter(|tile| **tile != 0)
                .count(),
            1
        );

        let tile_value = *grid
            .tiles
            .iter()
            .flatten()
            .filter(|tile| **tile != 0)
            .next()
            .expect("exist");
        assert!(tile_value == 2 || tile_value == 4);

        let mut grid = Grid {
            tiles: [[2; 4]; 4],
            ..Default::default()
        };

        if let Ok(()) = grid.random_spawn_tile() {
            panic!("should fail");
        }

        Ok(())
    }

    #[test]
    fn rotate() {
        let mut grids = vec![
            Grid {
                tiles: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
                ..Default::default()
            };
            4
        ];

        for (n_rotation, grid) in grids.iter_mut().enumerate() {
            grid.rotate_90_deg(n_rotation.try_into().expect("value fit"));
        }

        assert_eq!(
            grids[0],
            Grid {
                tiles: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
                ..Default::default()
            }
        );

        assert_eq!(
            grids[1],
            Grid {
                tiles: [[3, 6, 9], [2, 5, 8], [1, 4, 7]],
                ..Default::default()
            }
        );

        assert_eq!(
            grids[2],
            Grid {
                tiles: [[9, 8, 7], [6, 5, 4], [3, 2, 1]],
                ..Default::default()
            }
        );

        assert_eq!(
            grids[3],
            Grid {
                tiles: [[7, 4, 1], [8, 5, 2], [9, 6, 3]],
                ..Default::default()
            }
        );
    }

    const GRID_BEFORE_MOVE: Grid<4> = Grid {
        tiles: [[2, 0, 0, 0], [0, 2, 2, 0], [2, 4, 8, 16], [16, 8, 4, 2]],
        score: 0,
    };

    const GRID_AFTER_MOVE_RIGHT: Grid<4> = Grid {
        tiles: [[0, 0, 0, 2], [0, 0, 0, 4], [2, 4, 8, 16], [16, 8, 4, 2]],
        score: 4,
    };

    #[test]
    fn move_tiles_up() {
        let mut grid = GRID_BEFORE_MOVE.clone();
        grid.rotate_90_deg(1);
        grid.move_tiles(MoveDirection::Up);

        let mut result = GRID_AFTER_MOVE_RIGHT.clone();
        result.rotate_90_deg(1);

        assert_eq!(grid, result);
    }

    #[test]
    fn move_tiles_down() {
        let mut grid = GRID_BEFORE_MOVE.clone();
        grid.rotate_90_deg(-1);
        grid.move_tiles(MoveDirection::Down);

        let mut result = GRID_AFTER_MOVE_RIGHT.clone();
        result.rotate_90_deg(-1);

        assert_eq!(grid, result);
    }

    #[test]
    fn move_tiles_right() {
        let mut grid = GRID_BEFORE_MOVE.clone();
        grid.move_tiles(MoveDirection::Right);

        assert_eq!(grid, GRID_AFTER_MOVE_RIGHT);
    }

    #[test]
    fn move_tiles_left() {
        let mut grid = GRID_BEFORE_MOVE.clone();
        grid.rotate_90_deg(2);
        grid.move_tiles(MoveDirection::Left);

        let mut result = GRID_AFTER_MOVE_RIGHT.clone();
        result.rotate_90_deg(2);

        assert_eq!(grid, result);
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
