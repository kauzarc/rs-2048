use rs_2048::grid::Grid;

fn main() {
    let grid = Grid::<4>::default();

    println!("{}", grid);
    println!("{}", grid.random_spawn_tile().unwrap());
}
