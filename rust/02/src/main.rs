use std::{fmt, thread, time};

#[derive(Debug)]
enum State {
    Alive,
    Dead,
}

#[derive(Debug)]
struct Position(i32, i32);

impl Position {
    fn from(other: &Self) -> Self {
        Self(other.0, other.1)
    }
}

#[derive(Debug)]
struct Cell {
    position: Position,
    state: State,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state {
            State::Alive => write!(f, "O"),
            State::Dead => write!(f, " "),
        }
    }
}

#[derive(Debug)]
struct Board {
    pub cells: Vec<Cell>,
    width: i32,
    height: i32,
}

impl Board {
    fn new(width: i32, height: i32) -> Self {
        let mut cells = Vec::new();

        for x in 1..width {
            for y in 1..height {
                cells.push(Cell {
                    position: Position(x, y),
                    state: if rand::random::<f64>() < 0.2 {
                        State::Alive
                    } else {
                        State::Dead
                    },
                })
            }
        }

        Self {
            cells,
            height,
            width,
        }
    }

    fn tick(&mut self) {
        let mut cells = Vec::new();
        let alive_cells: Vec<&Cell> = self
            .cells
            .iter()
            .filter(|c| matches!(c.state, State::Alive))
            .collect();
        for cell in &self.cells[..] {
            let mut neighbors_count = 0;
            for c in &alive_cells[..] {
                if c.position.1 == cell.position.1 - 1 {
                    if c.position.0 == cell.position.0 - 1 {
                        neighbors_count += 1
                    }
                    if c.position.0 == cell.position.0 {
                        neighbors_count += 1
                    }
                    if c.position.0 == cell.position.0 + 1 {
                        neighbors_count += 1
                    }
                    continue;
                }
                if c.position.1 == cell.position.1 {
                    if c.position.0 == cell.position.0 - 1 {
                        neighbors_count += 1
                    }
                    if c.position.0 == cell.position.0 + 1 {
                        neighbors_count += 1
                    }
                    continue;
                }
                if c.position.1 == cell.position.1 + 1 {
                    if c.position.0 == cell.position.0 - 1 {
                        neighbors_count += 1
                    }
                    if c.position.0 == cell.position.0 {
                        neighbors_count += 1
                    }
                    if c.position.0 == cell.position.0 + 1 {
                        neighbors_count += 1
                    }
                    continue;
                }
            }

            let state = match cell.state {
                State::Alive => {
                    if neighbors_count == 2 || neighbors_count == 3 {
                        State::Alive
                    } else {
                        State::Dead
                    }
                }
                State::Dead => {
                    if neighbors_count == 3 {
                        State::Alive
                    } else {
                        State::Dead
                    }
                }
            };

            cells.push(Cell {
                state,
                position: Position::from(&cell.position),
            })
        }
        self.cells = cells;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = vec![vec![String::new(); self.width as usize]; self.height as usize];
        for cell in &self.cells {
            grid[(cell.position.1 - 1) as usize][(cell.position.0 - 1) as usize] = cell.to_string()
        }
        for line in grid {
            write!(f, "{}\n", line.join("")).unwrap();
        }

        Ok(())
    }
}

fn main() {
    let width = 150;
    let height = 50;

    let mut board = Board::new(width, height);

    let sleep_time = time::Duration::from_millis(100);
    let esc = 27 as char;
    loop {
        print!("{esc}c"); // \033c clears the terminal
        println!("{board}");
        board.tick();
        thread::sleep(sleep_time);
    }
}
