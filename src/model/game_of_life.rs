use rand::Rng;
use std::collections::HashSet;
use std::{fmt, thread, time};

pub struct GameOfLife {
    alive_cells: HashSet<(i32, i32)>,
    step: u32,
}

impl GameOfLife {
    pub fn new() -> Self {
        GameOfLife {
            alive_cells: HashSet::new(),
            step: 0,
        }
    }

    pub fn randomize(&mut self, w: Option<usize>, h: Option<usize>) {
        let mut rng = rand::rng();

        let width = w.unwrap_or(30);
        let height = h.unwrap_or(30);

        for i in 0..height {
            for j in 0..width {
                if rng.random::<f64>() < 0.2 {
                    self.alive_cells
                        .insert((i.try_into().unwrap(), j.try_into().unwrap()));
                }
            }
        }
    }

    pub fn randomize_area(&mut self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        let mut rng = rand::rng();

        for i in min_x..=max_x {
            for j in min_y..=max_y {
                if rng.random::<f64>() < 0.2 {
                    self.alive_cells
                        .insert((i, j));
                }
            }
        }
    }

    pub fn add_alive_cell(&mut self, pos_x: i32, pos_y: i32) {
        self.alive_cells.insert((pos_x, pos_y));
    }

    pub fn step(&mut self) {
        self.step += 1;

        let mut new_alive_cells = HashSet::new();

        for cell in self.alive_cells.iter() {
            // Check for rules 1 & 2 & 3
            let neighbours_count = self.count_alive_neighbours(cell.0, cell.1);
            if neighbours_count == 2 || neighbours_count == 3 {
                new_alive_cells.insert(*cell);
            }

            // check for rule 4
            for neighbour in self.get_neighbors(cell.0, cell.1) {
                let nc = self.count_alive_neighbours(neighbour.0, neighbour.1);
                if nc == 3 {
                    new_alive_cells.insert(neighbour);
                }
            }
        }

        self.alive_cells = new_alive_cells;
    }

    pub fn count_alive_neighbours(&self, pos_x: i32, pos_y: i32) -> u32 {
        let mut count = 0;
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if i == 0 && j == 0 {
                    continue;
                }
                if self.alive_cells.contains(&(pos_x - i, pos_y - j)) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn count_alive_cells(&self) -> usize {
        self.alive_cells.len()
    }

    pub fn get_steps_count(&self) -> u32 {
        self.step
    }

    fn get_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        vec![
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
        ]
    }

    pub fn step_delay(&self, msecs: u64) {
        thread::sleep(time::Duration::from_millis(msecs));
    }

    // Convert data into a readble output
    pub fn data_as_str(
        &self,
        mut min_x: i32,
        mut max_x: i32,
        mut min_y: i32,
        mut max_y: i32,
    ) -> String {
        if min_x > max_x {
            let aux = min_x;
            min_x = max_x;
            max_x = aux;
        }
        if min_y > max_y {
            let aux = min_y;
            min_y = max_y;
            max_y = aux;
        }
        let mut data_as_str: String = String::new();

        // Calculate screen size from input
        let width = (max_x - min_x + 1).try_into().unwrap();
        let height = (max_y - min_y + 1).try_into().unwrap();

        let mut output: Vec<Vec<char>> = vec![vec!('⬛'; width); height];

        // Filter only the cell in the region to draw
        for (x, y) in self
            .alive_cells
            .iter()
            .filter(|(a, b)| *a >= min_x && *a <= max_x && *b >= min_y && *b <= max_y)
        {
            // rotate for better diplay
            let pos_x: usize = (max_x - x).try_into().unwrap();
            let pos_y: usize = (y - min_y).try_into().unwrap();
            output[pos_x][pos_y] = '⬜';
        }

        for line in output {
            data_as_str.push_str(line.into_iter().collect::<String>().as_str());
            data_as_str.push('\n');
        }
        data_as_str
    }

    pub fn data_as_vec(&self, area: (i32, i32, i32, i32)) -> HashSet<(i32,i32)> {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = area;
        if min_x > max_x {
            let aux = min_x;
            min_x = max_x;
            max_x = aux;
        }
        if min_y > max_y {
            let aux = min_y;
            min_y = max_y;
            max_y = aux;
        }

        println!("({},{})({},{})", min_x,min_y,max_x,max_y);
        let mut output = HashSet::new();
        // Filter only the cell in the region to draw
        for (x, y) in self
            .alive_cells
            .iter()
            .filter(|(a, b)| *a >= min_x && *a <= max_x && *b >= min_y && *b <= max_y)
        {
            output.insert((*x, *y));
        }

        output
    }
}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n\nSTEP: {}\tALIVE CELLS: {}",
            self.data_as_str(1, 3, 1, 3),
            self.step,
            self.count_alive_cells()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_count_alive_neighbours() {
        let mut gol = GameOfLife::new();
        assert_eq!(gol.count_alive_neighbours(0, 0), 0);
        gol.add_alive_cell(0, 0);
        assert_eq!(gol.count_alive_neighbours(0, 0), 0);
        gol.add_alive_cell(1, 0);
        assert_eq!(gol.count_alive_neighbours(0, 0), 1);
        gol.add_alive_cell(2, 0);
        assert_eq!(gol.count_alive_neighbours(0, 0), 1);
        gol.add_alive_cell(1, 1);
        gol.add_alive_cell(0, 1);
        gol.add_alive_cell(-1, 1);
        gol.add_alive_cell(-1, 0);
        gol.add_alive_cell(-1, -1);
        gol.add_alive_cell(0, -1);
        gol.add_alive_cell(1, -1);
        assert_eq!(gol.count_alive_neighbours(0, 0), 8);
    }

    #[test]
    fn test_rule1() {
        let mut gol = GameOfLife::new();
        gol.add_alive_cell(0, 0);
        assert_eq!(gol.data_as_str(-1, 1, -1, 1), "⬛⬛⬛\n⬛⬜⬛\n⬛⬛⬛\n");
        gol.step();
        assert_eq!(gol.data_as_str(-1, 1, -1, 1), "⬛⬛⬛\n⬛⬛⬛\n⬛⬛⬛\n");
        gol.add_alive_cell(0, 0);
        gol.add_alive_cell(0, 1);
        assert_eq!(gol.data_as_str(-1, 1, -1, 1), "⬛⬛⬛\n⬛⬜⬜\n⬛⬛⬛\n");
        gol.step();
        assert_eq!(gol.data_as_str(-1, 1, -1, 1), "⬛⬛⬛\n⬛⬛⬛\n⬛⬛⬛\n");
    }

    #[test]
    fn test_rule2() {
        let mut gol = GameOfLife::new();

        gol.add_alive_cell(0, 0);
        gol.add_alive_cell(0, 1);
        gol.add_alive_cell(0, -1);

        assert_eq!(gol.data_as_str(-1, 1, -1, 1), "⬛⬛⬛\n⬜⬜⬜\n⬛⬛⬛\n");

        gol.step();
        // the cell in the center is still alive
        assert_eq!(gol.data_as_str(-1, 1, -1, 1), "⬛⬜⬛\n⬛⬜⬛\n⬛⬜⬛\n");
    }

    #[test]
    fn test_rule3() {
        let mut gol = GameOfLife::new();
        gol.add_alive_cell(2, 2);
        gol.add_alive_cell(1, 2);
        gol.add_alive_cell(2, 1);
        gol.add_alive_cell(3, 2);
        gol.add_alive_cell(2, 3);
        assert_eq!(gol.data_as_str(1, 3, 1, 3), "⬛⬜⬛\n⬜⬜⬜\n⬛⬜⬛\n");

        gol.step();
        // the center cell dies
        assert_eq!(gol.data_as_str(1, 3, 1, 3), "⬜⬜⬜\n⬜⬛⬜\n⬜⬜⬜\n");
    }

    #[test]
    fn test_rule4() {
        let mut gol = GameOfLife::new();
        gol.add_alive_cell(1, 1);
        gol.add_alive_cell(1, 2);
        gol.add_alive_cell(2, 1);

        assert_eq!(gol.data_as_str(1, 2, 1, 2), "⬜⬛\n⬜⬜\n");

        gol.step();
        // a new cell lives
        assert_eq!(gol.data_as_str(1, 2, 1, 2), "⬜⬜\n⬜⬜\n");
    }
}
