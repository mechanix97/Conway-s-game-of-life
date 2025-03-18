pub mod model;

use model::game_of_life::GameOfLife;

use clearscreen;





fn main() {
    let mut gol = GameOfLife::new();
    // gol.randomize(None, None);

    gol.add_alive_cell(2, 2);
    gol.add_alive_cell(1, 2);
    gol.add_alive_cell(2, 1);
    gol.add_alive_cell(3, 2);
    gol.add_alive_cell(2, 3);


    println!("{gol}");
        gol.step();

        println!("{gol}");

    // loop{ 
    //     clearscreen::clear().expect("failed to clear screen");
    //     println!("{gol}");
    //     gol.step();
    //     gol.step_delay(500);
      
    // }
    
}
