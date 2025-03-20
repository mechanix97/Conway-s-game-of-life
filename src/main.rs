pub mod model;
pub mod view;

use std::sync::{Arc, RwLock};
use std::thread;


use model::game_of_life::GameOfLife;
use view::screen::Screen;



#[macroquad::main("Conway's game of life")]
async fn main() {
    let mut screen = Screen::new();
    screen.set_area(-10, -10,40 , 40);
    let gol = Arc::new(RwLock::new(GameOfLife::new()));
    gol.write().unwrap().randomize(Some(30), Some(30));


    let mut running = true;

    let gol_clone = gol.clone();

    //Spawn the simulation in a new thread for better performace
    let join = thread::spawn(move || {
       while running{
            {gol_clone.write().unwrap().step();}
            gol_clone.read().unwrap().step_delay(100);
       }
    });

    loop {
        let _ = screen.check_buttons();
        let area = screen.get_area();
        let data;
        let step; 
        let cells_alive;
        {
            data = gol.read().unwrap().data_as_vec(area);
            step = gol.read().unwrap().get_steps_count();
            cells_alive = gol.read().unwrap().count_alive_cells() as u32;
        }
        screen.draw_frame(data, step, cells_alive).await;
    }

    running = false;
    let _ = join.join();
}
