use std::collections::HashSet;

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

    // receives a hashset indicating the cords of the alive cells
    pub async fn draw_frame(&mut self, gol_data: HashSet<(i32,i32)>) {
        clear_background(WHITE);

        let rows = self.posy_max.abs() - self.posy_min;
        let cols = self.posx_max.abs() - self.posx_min; 

        let cell_heigth: f32 = (screen_height() - 30.0)/ (rows as f32);
        let cell_width: f32 = screen_width() / (cols as f32);

        for i in 0..rows {
            for j in 0..cols {
                if gol_data.contains(&(i,j)) {
                    draw_rectangle(
                        (j as f32) * cell_width,
                        (i as f32) * cell_heigth,
                        cell_width,
                        cell_heigth,
                        BLACK,
                    );
                }  
            }
        }
        self.draw_footer();
        next_frame().await
    }


    pub fn draw_footer(&self) {
        draw_rectangle(
            0.0,
            screen_height() - 30.0,
            screen_width(),
            30.0,
            GRAY,
        );
        draw_text(format!("STEP: {}     CELLS ALIVE: {}", 5, 12).as_str(), 5.0 ,screen_height() - 7.0 , 25.0, BLACK);
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
