use std::{thread, time::Duration};
use raylib::prelude::*;

struct State<const N: usize, const M: usize> {
    cells: [[u8; N]; M],
}

impl<const N: usize, const M: usize> State<N, M> {
    fn new() -> Self {
        let mut new_cells: [[u8; N]; M] =[[0 as u8; N]; M];
        new_cells[1][0] = 1;
        new_cells[2][1] = 1;
        new_cells[0][2] = 1;
        new_cells[1][2] = 1;
        new_cells[2][2] = 1;
        let new_state = Self {
            cells: new_cells.to_owned(),
        };
        return new_state;
    }

    fn get_cells(&self) -> [[u8; N]; M] {
        return self.cells;
    }

    fn step(&mut self) {
        let mut next_state: [[u8; N]; M] = [[0 as u8; N]; M];
        for (i, row) in self.cells.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let neighbors = State::<N, M>::count_neighbors(&self, i as i32, j as i32);
                if neighbors == 3 {
                    next_state[i][j] = 1;
                } else if neighbors == 2 {
                    next_state[i][j] = *col;
                } else {
                    next_state[i][j] = 0;
                }

            }
        }
        self.cells = next_state.clone();
    }

    fn count_neighbors(&self, i: i32, j: i32) -> u8 {
        let mut alive_neighbors = 0;
        for di in 0..=2 {
            for dj in 0..=2 {
                if di != 1 || dj != 1 {
                    let y = State::<N, M>::modu(i + (di - 1), M as i32);
                    let x = State::<N, M>::modu(j + (dj - 1), N as i32);
                    alive_neighbors += self.cells[y][x];
                }
            }
        }
        return alive_neighbors;
    }

    fn modu(a: i32, b: i32) -> usize {
        return (((a % b) + b) % b) as usize;
    }
}

const UNIT_SIZE: usize = 10; // the graphical size of a cell
const PADDING: usize = 12;
const FONT_SIZE: usize = (3 * UNIT_SIZE) - (PADDING / 2);
const TITLE_OFFSET: usize = FONT_SIZE + PADDING;

const SCREEN_WIDTH: f32 = 128 as f32 * UNIT_SIZE as f32;
const SCREEN_HEIGHT: f32 = 96 as f32 * UNIT_SIZE as f32;

const WIDTH: usize = (SCREEN_WIDTH as i32 / UNIT_SIZE as i32) as usize;
const HEIGHT: usize = ((SCREEN_HEIGHT as i32 - TITLE_OFFSET as i32) / UNIT_SIZE as i32) as usize;

fn main() {
    println!("Hello Graphigol");
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Graphigol")
        .build();


    let mut game: State<HEIGHT, WIDTH> = State::<HEIGHT, WIDTH>::new();


    while !rl.window_should_close() {
        /* UPDATE STATE */
        game.step();

        /* RENDER FRAME */
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("Graphigol", PADDING as i32, PADDING as i32, FONT_SIZE as i32, Color::WHITE);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {

                if game.get_cells()[x][y] == 1 {
                    let cell_x: i32 = translate_x(x);
                    let cell_y: i32 = translate_y(y);
                    d.draw_rectangle(cell_x, cell_y, UNIT_SIZE as i32, UNIT_SIZE as i32, Color::WHITE);
                }
            }
        }
        thread::sleep(Duration::from_millis(32));
    }
}

fn translate_x(x: usize) -> i32 {
    (x * UNIT_SIZE) as i32
}
fn translate_y(y: usize) -> i32 {
    (y * UNIT_SIZE) as i32 + TITLE_OFFSET as i32
}
