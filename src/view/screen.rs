use macroquad::prelude::*;


pub struct Screen{

}


impl Screen{
    pub fn new() -> Self {
        Screen{}
    }


    
    pub async fn main_loop(&mut self){
        let mut y: f32 = 40.0;    
        let mut t = 0.0;
        loop{
            clear_background(WHITE);

            draw_line(40.0, y, 100.0, y+160.0, 15.0, BLUE);
            draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
            draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

            draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
            t += 0.01;
            y = y + t*t;
            next_frame().await
        }
    }
}