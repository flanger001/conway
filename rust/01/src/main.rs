use std::{fmt, thread, time};

#[derive(Debug)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(length: u8, width: u8) -> Board {
        let mut cells = Vec::new();

        for _ in 0..length {
            let mut row = Vec::new();

            for _ in 0..width {
                let cell = if rand::random::<f32>() < 0.2 {
                    Cell::Alive
                } else {
                    Cell::Dead
                };

                row.push(cell)
            }

            cells.push(row);
        }

        Board { cells }
    }

    pub fn tick(&mut self) {
        let mut cells = Vec::new();
        let max_row_count = self.cells.len();

        for y in 0..max_row_count {
            let mut new_row = Vec::new();
            let row = &self.cells[y];

            for (x, cell) in row.iter().enumerate() {
                let neighbor_count = self.count_neighbors(x, y);
                let new_cell = match cell {
                    Cell::Alive => {
                        if neighbor_count == 2 || neighbor_count == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                    Cell::Dead => {
                        if neighbor_count == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                };

                new_row.push(new_cell)
            }

            cells.push(new_row)
        }

        self.cells = cells
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let max_row_count = self.cells.len() - 1;
        let mut neighbor_count = 0;
        neighbor_count += self.neighbors_adjacent(x, y);

        if y == 0 {
            // At top of board
            neighbor_count += self.neighbors_below(x, y);
        } else if y < max_row_count - 1 {
            // Middle of board
            neighbor_count += self.neighbors_above(x, y);
            neighbor_count += self.neighbors_below(x, y);
        } else {
            // Bottom of board
            neighbor_count += self.neighbors_above(x, y);
        }
        neighbor_count
    }

    fn neighbors_adjacent(&self, x: usize, y: usize) -> u8 {
        let mut neighbor_count = 0;
        let row_length = self.cells[y].len();

        // If we are not at the left edge
        if x > 0 {
            match self.cells[y][x - 1] {
                Cell::Alive => neighbor_count += 1,
                Cell::Dead => (),
            }
        }

        // If we are not at the right edge
        if x < row_length - 1 {
            match self.cells[y][x + 1] {
                Cell::Alive => neighbor_count += 1,
                Cell::Dead => (),
            }
        }

        neighbor_count
    }

    fn neighbors_above(&self, x: usize, y: usize) -> u8 {
        let mut neighbor_count = match self.cells[y - 1][x] {
            Cell::Alive => 1,
            Cell::Dead => 0,
        };

        neighbor_count += self.neighbors_adjacent(x, y - 1);
        neighbor_count
    }

    fn neighbors_below(&self, x: usize, y: usize) -> u8 {
        let mut neighbor_count = match self.cells[y + 1][x] {
            Cell::Alive => 1,
            Cell::Dead => 0,
        };

        neighbor_count += self.neighbors_adjacent(x, y + 1);
        neighbor_count
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Cell {
    Alive,
    Dead,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alive => write!(f, "O"),
            Self::Dead => write!(f, " "),
        }
    }
}

fn main() {
    let mut board = Board::new(50, 150);
    loop {
        print!("{esc}c", esc = 27 as char); // \033c clears the terminal

        board.tick();
        println!("{board}");

        let sleep_time = time::Duration::from_millis(100);

        thread::sleep(sleep_time);
    }
}
