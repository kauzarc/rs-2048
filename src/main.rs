use rs_2048::grid::{Grid, MoveDirection};
use std::io;

fn main() -> anyhow::Result<()> {
    let mut grid = Grid::<4>::default();

    while !grid.game_over() {
        grid.random_spawn_tile()?;

        println!("{}", grid);

        let mut buffer = String::new();

        while !["z", "q", "s", "d"].iter().any(|key| buffer == **key) {
            println!("Enter direction (z, q, s, d) :");

            let stdin = io::stdin();

            buffer.clear();
            stdin.read_line(&mut buffer)?;
        }

        grid.move_tiles(match buffer.as_str() {
            "z" => MoveDirection::Up,
            "q" => MoveDirection::Left,
            "s" => MoveDirection::Down,
            "d" => MoveDirection::Right,
            _ => panic!(),
        });
    }

    Ok(())
}
