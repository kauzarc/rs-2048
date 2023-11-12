use rand::{self, distributions, prelude::Distribution, seq::SliceRandom};

#[derive(Debug, Clone)]
pub struct Grid<const N: usize> {
    tiles: [[Option<Tile>; N]; N],
    score: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub value: u32,
    pub position: TilePosition,
}

#[derive(Debug, Clone, Copy)]
pub struct TilePosition {
    pub i: usize,
    pub j: usize,
}

impl<const N: usize> Default for Grid<N> {
    fn default() -> Self {
        Self {
            tiles: [[None; N]; N],
            score: Default::default(),
        }
    }
}

impl<const N: usize> Grid<N> {
    pub fn is_full(&self) -> bool {
        self.tiles.iter().flatten().all(|tile| tile.is_some())
    }

    pub fn random_spawn_tile(&self) -> Option<Self> {
        let mut result = self.clone();
        let mut empty_tiles: Vec<(TilePosition, &mut Option<Tile>)> = result
            .tiles
            .iter_mut()
            .enumerate()
            .map(|(i, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(j, tile)| (TilePosition { i, j }, tile))
            })
            .flatten()
            .filter(|(_, tile)| tile.is_none())
            .collect();

        let mut rng = rand::thread_rng();
        let (position, tile) = empty_tiles.choose_mut(&mut rng)?;

        let dist = distributions::Bernoulli::new(0.75).ok()?;
        **tile = Some(Tile {
            value: if dist.sample(&mut rng) { 2 } else { 4 },
            position: *position,
        });

        Some(result)
    }

    pub fn tiles(&self) -> &[[Option<Tile>; N]; N] {
        &self.tiles
    }

    pub fn score(&self) -> &u32 {
        &self.score
    }
}