use anyhow::Result;
use rs_2048::grid::{Grid, MoveDirection};

fn main() -> Result<()> {
    let mut grid = Grid::<4>::default();

    println!("{}", grid);

    grid.random_spawn_tile()?;
    println!("{}", grid);

    grid.move_tiles(MoveDirection::Right)?;
    println!("{}", grid);

    Ok(())
}
