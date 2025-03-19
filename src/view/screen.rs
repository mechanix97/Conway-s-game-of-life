use macroquad::prelude::*;

//pos* indicates the area of the simulation to show in the screen

pub struct Screen {
    posx_min: i32,
    posy_min: i32,
    posx_max: i32,
    posy_max: i32,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            posx_min: 0,
            posx_max: 0,
            posy_min: 0,
            posy_max: 0,
        }
    }

    // receives a matrix of bools indicating the state of the cells
    pub async fn draw_frame(&mut self, gol_data: Vec<Vec<bool>>) {
        clear_background(WHITE);

        let cell_heigth: f32 = screen_height() / (gol_data.len() as f32);
        let cell_width: f32 = screen_width() / (gol_data[0].len() as f32);

        for iu in 0..gol_data.len() {
            for ju in 0..gol_data[0].len() {
                let color = match gol_data[iu][ju] {
                    true => BLACK,
                    false => WHITE
                };
                let i: f32 = iu as f32;
                let j: f32 = ju as f32;

                draw_rectangle(
                    j * cell_width,
                    i * cell_heigth,
                    (j + 1.0) * cell_width,
                    (i + 1.0) * cell_heigth,
                    color,
                );
            }
        }
        next_frame().await
    }

    // check if a button has been pressed
    // for arrows, move the view by 10% in a given direction
    pub fn check_buttons(&mut self) -> bool {
        let mut refresh = false;
        if is_key_down(KeyCode::Left) {
            refresh = true;
            let mut mov = (self.posy_max.abs() - self.posy_min) / 20;
            if mov == 0 {
                mov = 1;
            }
            self.posy_max -= mov;
            self.posy_min -= mov;
        } else if is_key_down(KeyCode::Right) {
            refresh = true;
            let mut mov = (self.posy_max.abs() - self.posy_min) / 20;
            if mov == 0 {
                mov = 1;
            }
            self.posy_max += mov;
            self.posy_min += mov;
        }
        if is_key_down(KeyCode::Down) {
            refresh = true;
            let mut mov = (self.posx_max.abs() - self.posx_min) / 20;
            if mov == 0 {
                mov = 1;
            }
            self.posx_max -= mov;
            self.posx_min -= mov;
        } else if is_key_down(KeyCode::Up) {
            refresh = true;
            let mut mov = (self.posx_max.abs() - self.posx_min) / 20;
            if mov == 0 {
                mov = 1;
            }
            self.posx_max += mov;
            self.posx_min += mov;
        }

        if is_key_down(KeyCode::Minus){
            refresh = true;
            let mut mov = (self.posy_max.abs() - self.posy_min) / 20;
            if mov == 0 {
                mov = 1;
            }
            self.posy_max += mov;
            self.posy_min -= mov;
            mov = (self.posx_max.abs() - self.posx_min) / 20;
            if mov == 0 {
                mov = 1;
            }
            self.posx_max += mov;
            self.posx_min -= mov;

        }
        if is_key_down(KeyCode::Equal){
            refresh = true;
            let mut mov = (self.posy_max.abs() - self.posy_min) / 20;
            if mov == 0 {
                mov = 1;
            }
            self.posy_max -= mov;
            self.posy_min += mov;
            mov = (self.posx_max.abs() - self.posx_min) / 20;
            if mov == 0 {
                mov = 1;
            }
            self.posx_max -= mov;
            self.posx_min += mov;
        }

        refresh
    }

    pub fn set_area(&mut self, a: i32, b: i32, c: i32, d: i32) {
        self.posx_min = a;
        self.posy_min = b;
        self.posx_max = c;
        self.posy_max = d;
    }

    pub fn get_area(&self) -> (i32, i32, i32, i32) {
        (self.posx_min, self.posy_min, self.posx_max, self.posy_max)
    }
}
