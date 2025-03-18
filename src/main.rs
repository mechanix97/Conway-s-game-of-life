pub mod model;

use model::game_of_life::GameOfLife;

use clearscreen;





fn main() {
    let mut gol = GameOfLife::new();

    loop{
        gol.randomize(None, None);
        clearscreen::clear().expect("failed to clear screen");
        println!("{gol}");
        gol.step();
        gol.step_delay(500);
      
    }
    
}
