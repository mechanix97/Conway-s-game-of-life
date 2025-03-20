pub mod model;
pub mod view;

use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use macroquad::prelude::*;

use model::game_of_life::GameOfLife;
use view::screen::Screen;

#[macroquad::main("Conway's game of life")]
async fn main() {
    prevent_quit();

    let mut screen = Screen::new();
    screen.set_area(-20, -20, 20, 20);
    let gol = Arc::new(RwLock::new(GameOfLife::new()));
    gol.write().unwrap().randomize_area(-20, -20, 20, 20);

    let paused = Arc::new(AtomicBool::new(false));
    let running = Arc::new(AtomicBool::new(true));

    
    let gol_clone = gol.clone();
    let paused_clone = paused.clone();
    let running_clone = running.clone();

    //Spawn the simulation in a new thread for better performace
    let join_handle = thread::spawn(move || {
        loop {
            if !running_clone.load(Ordering::Relaxed){
                break;
            }
            if !paused_clone.load(Ordering::Relaxed) {
                gol_clone.write().unwrap().step();
            }
            gol_clone.read().unwrap().step_delay(100);
        }
    });
    
    loop {
        if is_quit_requested() {
            running.store(false, Ordering::Relaxed);
            break;
        }
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
        if paused.load(Ordering::Relaxed) != screen.is_paused(){
            paused.store(screen.is_paused(), Ordering::Relaxed, );
        }
        screen.draw_frame(data, step, cells_alive).await;
    }

    join_handle.join().unwrap();
}
