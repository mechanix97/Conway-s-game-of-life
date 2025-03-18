use rand::Rng;
use std::{fmt, thread, time};


#[derive(Clone, PartialEq)]
enum LifeStatus{
    Alive,
    Dead
}

pub struct GameOfLife{
    data: Vec<Vec<LifeStatus>>,
    step: u32
}

impl GameOfLife{
    pub fn new() -> Self {
        GameOfLife{
            data: vec!(vec!()),
            step: 0
        }
    }

    pub fn randomize(&mut self, w: Option<usize>, h: Option<usize>) {
        let mut rng = rand::rng();

        let width = w.unwrap_or(30);
        let height = h.unwrap_or(30);
        self.data = vec!(vec!(LifeStatus::Dead; width.try_into().unwrap()); height.try_into().unwrap());

        for i in 0..height{
            for j in 0..width{
                if rng.random::<f64>() < 0.2 {
                    self.data[i][j] = LifeStatus::Alive;
                }
            }
        }
    }

    pub fn step(&mut self){
        self.step += 1;
    }

    fn count_alive_neighbours(&self, i: usize, j: usize) -> u32{
        0
    }

    fn count_alive_cells(&self) -> i32{
        let mut count = 0;
        for line in &self.data{
            for v in line{
                if *v == LifeStatus::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    
    pub fn step_delay(&mut self, secs: u64){
        thread::sleep(time::Duration::from_secs(secs));        
    }

    // Convert data into a readble output 
    pub fn data_as_str(&self) -> String {
        let mut data_as_str: String = String::new();
        for line in &self.data {
            for v in line {
                data_as_str.push(
                    match v {
                        LifeStatus::Alive => '⬜',
                        LifeStatus::Dead => '⬛'
                    }
                );
            }
            data_as_str.push('\n');
        }
        data_as_str
    }
}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{}\n\nSTEP: {}\tALIVE CELLS: {}",self.data_as_str(), self.step, self.count_alive_cells())
    }
}
