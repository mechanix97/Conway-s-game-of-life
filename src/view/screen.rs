use std::collections::HashSet;

use macroquad::prelude::*;

const MOVEMENT_RATE: f32 = 0.01;

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
    pub async fn draw_frame(&mut self, gol_data: HashSet<(i32,i32)> , step: u32, cells_alive: u32) {
        clear_background(WHITE);

        let rows = self.posy_max.abs() - self.posy_min;
        let cols = self.posx_max.abs() - self.posx_min; 

        let cell_heigth: f32 = (screen_height() - 30.0)/ (rows as f32);
        let cell_width: f32 = screen_width() / (cols as f32);

        for cell in gol_data {
            draw_rectangle(
                (cell.1 as f32) * cell_width,
                (cell.0 as f32) * cell_heigth,
                cell_width,
                cell_heigth,
                BLACK,
            );
        }
        self.draw_footer(step, cells_alive);
        next_frame().await
    }


    pub fn draw_footer(&self, step: u32, cells_alive: u32) {
        draw_rectangle(
            0.0,
            screen_height() - 30.0,
            screen_width(),
            30.0,
            GRAY,
        );
        let posx_mid = (self.posx_max + self.posx_min)/2;
        let posy_mid = (self.posy_max + self.posy_min)/2;

        draw_text(
            format!("STEP: {}     CELLS ALIVE: {}          POS: ({},{})", step, cells_alive, posx_mid, posy_mid).as_str(), 
            5.0 ,screen_height() - 7.0 , 25.0, BLACK);
    }

    // check if a button has been pressed
    // for arrows, move the view by a rate in a given direction
    pub fn check_buttons(&mut self) {
        let mut mov_x = ((self.posy_max.abs() - self.posy_min) as f32 * MOVEMENT_RATE) as i32;
        let mut mov_y = ((self.posy_max.abs() - self.posy_min) as f32 * MOVEMENT_RATE) as i32;

        if mov_x == 0 {mov_x=1;}
        if mov_y == 0 {mov_y=1;}

        if is_key_down(KeyCode::Left) {
            self.posy_max -= mov_y;
            self.posy_min -= mov_y;
        } else if is_key_down(KeyCode::Right) {
            self.posy_max += mov_y;
            self.posy_min += mov_y;
        }
        if is_key_down(KeyCode::Down) {
            self.posx_max -= mov_x;
            self.posx_min -= mov_x;
        } else if is_key_down(KeyCode::Up) {
            self.posx_max += mov_x;
            self.posx_min += mov_x;
        }

        if is_key_down(KeyCode::Minus){
            self.posy_max += mov_y;
            self.posy_min -= mov_y;
            self.posx_max += mov_x;
            self.posx_min -= mov_x;

        } else if is_key_down(KeyCode::Equal){
            if self.posy_max - self.posy_min > 1 {
                self.posy_max -= mov_y;
                self.posy_min += mov_y;
            }
            if self.posx_max - self.posx_min > 1 {
                self.posx_max -= mov_x;
                self.posx_min += mov_x;
            }
        }
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
