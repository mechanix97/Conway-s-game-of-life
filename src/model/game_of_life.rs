use std::fmt; 


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
            data: vec!(
                vec!(LifeStatus::Alive, LifeStatus::Dead,LifeStatus::Alive ),
                vec!(LifeStatus::Dead, LifeStatus::Alive,LifeStatus::Dead ),
                vec!(LifeStatus::Alive, LifeStatus::Dead,LifeStatus::Alive ),
            ),
            step: 0
        }
    }

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
        
        write!(f, "{}\n\nSTEP: {}",self.data_as_str(), self.step)
    }
}
