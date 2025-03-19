pub mod model;
pub mod view;

use model::game_of_life::{self, GameOfLife};
use view::screen::Screen;

#[macroquad::main("Conway's game of life")]
async fn main() {
    let mut screen = Screen::new();
    screen.set_area(0, 0, 30, 30);
    let mut gol = GameOfLife::new();
    gol.randomize(Some(30), Some(30));

    loop {
        let _ = screen.check_buttons();
        let area = screen.get_area();
        screen.draw_frame(gol.data_as_vec(area)).await;
        gol.step();
        gol.step_delay(100);
    }
}
