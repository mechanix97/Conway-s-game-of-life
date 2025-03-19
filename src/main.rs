pub mod view;
use view::screen::Screen;

#[macroquad::main("BasicShapes")]
async fn main() {

   let mut screen = Screen::new();
   screen.main_loop().await
}