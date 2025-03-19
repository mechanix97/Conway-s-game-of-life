pub mod model;
pub mod view;

use std::sync::{Arc, RwLock};
use std::thread;


use model::game_of_life::GameOfLife;
use view::screen::Screen;



#[macroquad::main("Conway's game of life")]
async fn main() {
    let mut screen = Screen::new();
    screen.set_area(-10, -10,110 , 110);
    let mut gol = Arc::new(RwLock::new(GameOfLife::new()));
    gol.write().unwrap().randomize(Some(100), Some(100));


    let mut running = true;

    let mut gol_clone = gol.clone();
    let mut join = thread::spawn(move || {
       while running{
            {gol_clone.write().unwrap().step();}
            gol_clone.read().unwrap().step_delay(100);
       }
    });

    loop {
        let _ = screen.check_buttons();
        let area = screen.get_area();
        let data;
        {data = gol.read().unwrap().data_as_vec(area);}
        screen.draw_frame(data).await;
    }

    running = false;
}
