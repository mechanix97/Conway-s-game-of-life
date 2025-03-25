use std::collections::HashSet;

use macroquad::prelude::*;

const MOVEMENT_RATE: f32 = 0.01;

const FOOTER_HEIGHT: f32 = 30.0;

//pos* indicates the area of the simulation to show in the screen
pub struct Screen {
    posx_min: i32,
    posy_min: i32,
    posx_max: i32,
    posy_max: i32,
    paused: bool,
    reset: bool,
    randomize: bool,
    cols: i32,
    rows: i32,
    cell_width: f32,
    cell_heigth: f32
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            posx_min: 0,
            posx_max: 0,
            posy_min: 0,
            posy_max: 0,
            paused: true,
            reset: false,
            randomize: false,
            cols: 0,
            rows: 0,
            cell_width: 0.0,
            cell_heigth: 0.0
        }
    }

    /// receives a hashset indicating the coords of the alive cells
    /// the coords are relative to the bottom left corner
    pub async fn draw_frame(&mut self, gol_data: HashSet<(i32, i32)>, step: u32, cells_alive: u32) {
        clear_background(WHITE);
        if gol_data.len() == 1 {
            draw_rectangle(0.0, 0.0, self.cell_width, self.cell_heigth, BLACK);
        }

        self.cell_heigth = (screen_height() - FOOTER_HEIGHT) / (self.rows as f32);
        self.cell_width = screen_width() / (self.cols as f32); 

        // draw the cells
        for cell in &gol_data {
            let px = (cell.0 - self.posx_min - 1) as f32;
            let py = (self.posy_max - cell.1) as f32;

            draw_rectangle(
                px * self.cell_width,
                py * self.cell_heigth,
                self.cell_width,
                self.cell_heigth,
                BLACK,
            );
        }
        // check if paused
        if self.paused {
            self.draw_pause_icon();
        }

        // draw the mouse hover
        // if the position has a cell, change the colors
        if let Some(mouse_poition) = self.get_mouse_position(){
            let px = mouse_poition.0 as i32 + self.posx_min as i32 + 1;
            let py = self.posy_max as i32 - mouse_poition.1 as i32;
            let (color1,color2) = match gol_data.contains(&(px, py)) {
                true => (GRAY, BLACK),
                false => (BLACK, WHITE)
            };
            
            draw_rectangle(
                (mouse_poition.0 as f32) * self.cell_width,
                (mouse_poition.1 as f32) * self.cell_heigth,
                self.cell_width,
                self.cell_heigth,
                color1,
            );
            let padding = (self.cell_width * 0.1, self.cell_heigth * 0.1);
            draw_rectangle(
                (mouse_poition.0 as f32) * self.cell_width + padding.0,
                (mouse_poition.1 as f32) * self.cell_heigth + padding.1,
                self.cell_width - (padding.0 * 2.0),
                self.cell_heigth - (padding.1 * 2.0),
                color2,
            );
        }

        // draw footer
        self.draw_footer(step, cells_alive);
        next_frame().await
    }

    pub fn draw_pause_icon(&self){
        draw_rectangle(20.0, 20.0, 10.0, 30.0, RED);
        draw_rectangle(40.0, 20.0, 10.0, 30.0, RED);
    }

    /// get the cell position if the mouse is on the screen
    pub fn get_mouse_position(&self) -> Option<(u32,u32)>{
        let mouse_position = mouse_position();

        match mouse_position {
            mouse_position if (
                mouse_position.0 > 0.0 &&
                mouse_position.1 > 0.0 &&
                mouse_position.0 < screen_width() &&
                mouse_position.1 < screen_height() - FOOTER_HEIGHT
            ) => {
                Some((
                    (mouse_position.0 / self.cell_width) as u32,
                    (mouse_position.1 / self.cell_heigth) as u32
                ))
            }
            _ => None

        }
    }

    pub fn draw_footer(&self, step: u32, cells_alive: u32) {
        draw_rectangle(0.0, screen_height() - FOOTER_HEIGHT, screen_width(), FOOTER_HEIGHT, GRAY);
        let posx_mid = (self.posx_max + self.posx_min) / 2;
        let posy_mid = (self.posy_max + self.posy_min) / 2;

        draw_text(
            format!("STEP: {}     CELLS ALIVE: {}", step, cells_alive).as_str(),
            5.0,
            screen_height() - 7.0,
            25.0,
            BLACK,
        );

        let pos_text = format!("POS:({},{})", posx_mid, posy_mid);
        draw_text(
            pos_text.as_str(),
            screen_width() - (pos_text.len() * 12) as f32,
            screen_height() - 7.0,
            25.0,
            BLACK,
        );
    }

    /// get the grid position of the mouse
    pub fn mouse_clicked_pos(&mut self) -> Option<(i32,i32)>{
        if is_mouse_button_pressed(MouseButton::Left) ||
        is_mouse_button_down(MouseButton::Right){
            return match self.get_mouse_position() {
                Some(pos) =>{
                    Some((
                        pos.0 as i32 + self.posx_min as i32 + 1,
                        self.posy_max as i32 - pos.1 as i32
                    )) 
                },
                None => None
            }
        } 
        None
    }

    /// check if a button has been pressed
    /// for arrows, move the view by a rate in a given direction
    /// I O for zoom
    /// P pause the game
    /// R reset the game
    /// C center the grid
    /// T randomize an areas
    pub fn check_buttons(&mut self) {
        let mut mov_y = match self.posy_max < 0 {
            true => ((self.posy_max - self.posy_min).abs() as f32 * MOVEMENT_RATE) as i32,
            false => ((self.posy_max - self.posy_min) as f32 * MOVEMENT_RATE) as i32,
        };
        
        let mut mov_x = match self.posx_max < 0 {
            true => ((self.posx_max - self.posx_min).abs() as f32 * MOVEMENT_RATE) as i32,
            false => ((self.posx_max - self.posx_min) as f32 * MOVEMENT_RATE) as i32,
        };
        
        // set a minimal value
        if mov_x == 0 {
            mov_x = 1;
        }
        if mov_y == 0 {
            mov_y = 1;
        }
        
        let mut refresh = false;

        if is_key_down(KeyCode::Down) {
            refresh = true;
            self.posy_max -= mov_y;
            self.posy_min -= mov_y;
        } else if is_key_down(KeyCode::Up) {
            refresh = true;
            self.posy_max += mov_y;
            self.posy_min += mov_y;
        }
        if is_key_down(KeyCode::Left) {
            refresh = true;
            self.posx_max -= mov_x;
            self.posx_min -= mov_x;
        } else if is_key_down(KeyCode::Right) {
            refresh = true;
            self.posx_max += mov_x;
            self.posx_min += mov_x;
        }
        // zoom out
        if is_key_down(KeyCode::O) {
            refresh = true;
            self.posy_max += mov_y;
            self.posy_min -= mov_y;
            self.posx_max += mov_x;
            self.posx_min -= mov_x;
        } else if is_key_down(KeyCode::I) { //zoom in
            refresh = true;
        
            let center_x = (self.posx_max + self.posx_min) / 2;
            let center_y = (self.posy_max + self.posy_min) / 2;
        
            if self.posy_max - self.posy_min > mov_y {
                self.posy_max -= mov_y;
                self.posy_min += mov_y;
            }
        
            if self.posx_max - self.posx_min > mov_x {
                self.posx_max -= mov_x;
                self.posx_min += mov_x;
            }
        
            let new_center_x = (self.posx_max + self.posx_min) / 2;
            let new_center_y = (self.posy_max + self.posy_min) / 2;
        
            self.posx_max += (center_x - new_center_x) as i32;
            self.posx_min += (center_x - new_center_x) as i32;
            self.posy_max += (center_y - new_center_y) as i32;
            self.posy_min += (center_y - new_center_y) as i32;
        }
        if is_key_pressed(KeyCode::P){
            self.paused = !self.paused;
        }
        if is_key_pressed(KeyCode::R){
            self.reset = true;
        }
        if is_key_pressed(KeyCode::T){
            self.randomize = true;
        }
        if is_key_pressed(KeyCode::C){
            refresh = true;
            self.posx_min = -20;
            self.posy_min = -20;
            self.posx_max = 20;
            self.posy_max = 20;
        }
        if refresh {
            self.rows = match self.posy_max < 0 {
                true => (self.posy_max - self.posy_min).abs()  as i32,
                false => (self.posy_max - self.posy_min) as i32,
            };

            self.cols = match self.posx_max < 0 {
                true => (self.posx_max - self.posx_min).abs()as i32,
                false => (self.posx_max - self.posx_min) as i32,
            };
    
            self.cell_heigth = (screen_height() - FOOTER_HEIGHT) / (self.rows as f32);
            self.cell_width = screen_width() / (self.cols as f32);    
        }
    }

    /// set the view area and change the aditional information of the screen
    pub fn set_area(&mut self, a: i32, b: i32, c: i32, d: i32) {
        self.posx_min = a;
        self.posy_min = b;
        self.posx_max = c;
        self.posy_max = d;

        self.rows = match self.posy_max < 0 {
            true => (self.posy_max - self.posy_min).abs()  as i32,
            false => (self.posy_max - self.posy_min) as i32,
        };

        self.cols = match self.posx_max < 0 {
            true => (self.posx_max - self.posx_min).abs()as i32,
            false => (self.posx_max - self.posx_min) as i32,
        };

        self.cell_heigth = (screen_height() - FOOTER_HEIGHT) / (self.rows as f32);
        self.cell_width = screen_width() / (self.cols as f32); 
    }

    pub fn get_area(&self) -> (i32, i32, i32, i32) {
        (self.posx_min, self.posy_min, self.posx_max, self.posy_max)
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_reset(&self) -> bool {
        self.reset
    }

    pub fn set_reset(&mut self, r: bool){
        self.reset = r;
    }

    pub fn is_random(&self) -> bool {
        self.randomize
    }

    pub fn set_random(&mut self, r: bool){
        self.randomize = r;
    }
    
}
