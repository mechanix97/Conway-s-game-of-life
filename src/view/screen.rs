use macroquad::prelude::*;

pub struct Screen {
    posx_min: i64,
    posx_max: i64,
    posy_min: i64,
    posy_max: i64,
    
}

impl Screen {
    pub fn new() -> Self {
        Screen {  
            posx_min: 0,
            posx_max: 0,
            posy_min: 0,
            posy_max: 0
        }
    }

    pub async fn draw_frame(&mut self, gol_data: Vec<Vec<char>>){
        clear_background(WHITE);

        let cell_width: f32 = screen_width()/(gol_data.len() as f32);
        let cell_heigth: f32 = screen_height()/(gol_data[0].len() as f32);  

        for iu in 0..gol_data.len(){
            for ju in 0..gol_data[0].len(){
                let mut color= match gol_data[iu][ju] {
                    '1' => BLACK,
                    '0' => WHITE,
                    _ => GREEN
                };
                let i: f32 = iu as f32;
                let j: f32 = ju as f32;
                println!("{} {} {} {}", i*cell_width, j*cell_heigth, (i+1.0)*cell_width, (j+1.0)*cell_heigth );
                draw_rectangle(i*cell_width, j*cell_heigth, (i+1.0)*cell_width, (j+1.0)*cell_heigth, color);        
            }
        }

        // self.draw()
        next_frame().await

    }


    pub fn check_buttons(&mut self ) -> bool {
        let mut refresh = false;
        if is_key_down(KeyCode::Down) {
            refresh = true;
            let mut mov = (self.posy_max.abs() - self.posy_min)/20;
            if mov == 0 {mov = 1;}
            self.posy_max -= mov;
            self.posy_min -= mov;
        } else if is_key_down(KeyCode::Up) {
            refresh = true;
            let mut mov = (self.posy_max.abs() - self.posy_min)/20;
            if mov == 0 {mov = 1;}
            self.posy_max -= mov;
            self.posy_min -= mov;
        }
        if is_key_down(KeyCode::Left) {
            refresh = true;
            let mut mov = (self.posx_max.abs() - self.posx_min)/20;
            if mov == 0 {mov = 1;}
            self.posx_max -= mov;
            self.posx_min -= mov;
        } else if is_key_down(KeyCode::Right) {
            refresh = true;
            let mut mov = (self.posx_max.abs() - self.posx_min)/20;
            if mov == 0 {mov = 1;}
            self.posx_max += mov;
            self.posx_min += mov;
        }
        
        refresh
    }

    pub async fn main_loop(&mut self) {
        let mut y: f32 = 40.0;
        let mut t = 0.0;
        loop {
            

            draw_line(40.0, y, 100.0, y + 160.0, 15.0, BLUE);
            draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
            draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

            draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
            t += 0.01;
            y = y + t * t;
            
        }
    }


}
