pub mod model;
pub mod view;

use macroquad::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use model::game_of_life::GameOfLife;
use view::screen::Screen;

const INITIAL_AREA: (i32, i32, i32, i32) = (-20, -20, 20, 20);
const RANDOMIZE_AREA: (i32, i32, i32, i32) = (-20, -20, 20, 20);

const SIMULATION_STEP_TIME: u64 = 100;

/// main function
/// connects the game logic with the view
/// could be in a separate file
/// but for the scope of this project, is ok to have it here
#[macroquad::main("Conway's game of life")]
async fn main() {
    // use this to avoid quit when the windows is closed
    prevent_quit();

    let mut screen = Screen::new();

    screen.set_area(
        INITIAL_AREA.0,
        INITIAL_AREA.1,
        INITIAL_AREA.2,
        INITIAL_AREA.3,
    );

    let gol = Arc::new(RwLock::new(GameOfLife::new()));
    let paused = Arc::new(AtomicBool::new(false));
    let running = Arc::new(AtomicBool::new(true));

    //clone the arcs for the new thread
    let gol_clone = gol.clone();
    let paused_clone = paused.clone();
    let running_clone = running.clone();

    //Spawn the simulation in a new thread for better performace
    let join_handle = thread::spawn(move || {
        //simulation loop
        loop {
            if !running_clone.load(Ordering::Relaxed) {
                break;
            }
            if !paused_clone.load(Ordering::Relaxed) {
                gol_clone.write().unwrap().step();
            }
            thread::sleep(Duration::from_millis(SIMULATION_STEP_TIME));
        }
    });

    // screen drawing loop
    loop {
        // check if screen is closed
        if is_quit_requested() {
            running.store(false, Ordering::Relaxed);
            break;
        }

        screen.check_buttons();
        let area = screen.get_area();
        let data;
        let step;
        let cells_alive;
        {
            data = gol.read().unwrap().data_as_vec(area);
            step = gol.read().unwrap().get_steps_count();
            cells_alive = gol.read().unwrap().count_alive_cells() as u32;
        }

        // p key pressed
        if paused.load(Ordering::Relaxed) != screen.is_paused() {
            paused.store(screen.is_paused(), Ordering::Relaxed);
        }

        // R key presed
        if screen.is_reset() {
            gol.write().unwrap().clear_cells();
            screen.set_reset(false);
        }

        // T key pressed
        if screen.is_random() {
            gol.write().unwrap().clear_cells();
            gol.write().unwrap().randomize_area(
                RANDOMIZE_AREA.0,
                RANDOMIZE_AREA.1,
                RANDOMIZE_AREA.2,
                RANDOMIZE_AREA.3,
            );
            screen.set_random(false);
        }

        // add/kill cell by clicking
        if let Some(pos) = screen.mouse_clicked_pos() {
            gol.write().unwrap().change_cell_status(pos.0, pos.1);
        }

        screen.draw_frame(data, step, cells_alive).await;
    }

    join_handle.join().unwrap();
}
