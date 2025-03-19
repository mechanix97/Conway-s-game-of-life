pub mod view;
pub mod model;

use view::screen::Screen;

#[macroquad::main("Conway's game of life")]
async fn main() {
    let mut screen = Screen::new();

    loop {
        let _ = screen.check_buttons();
        screen
            .draw_frame(vec![
                vec!['1', '0', '0', '0', '1'],
                vec!['0', '1', '0', '0', '1'],
                vec!['0', '1', '1', '0', '0'],
            ])
            .await
    }
}
