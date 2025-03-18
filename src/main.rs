pub mod model;

use model::game_of_life::GameOfLife;

fn main() {
    let  gol = GameOfLife::new();

     println!("{gol}");
}
