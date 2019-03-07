use term3d::*;

use std::cmp::{min, max};

const WORLD_WIDTH: usize = 32;
const WORLD_HEIGHT: usize = 32;

struct App {
    player_pos: (i32, i32),
    drawing: bool,
    direction: (i32, i32), // direction of movement: x, y; -1, 1; 0, 0; etc.
    map: [[char; WORLD_HEIGHT]; WORLD_WIDTH],
    viewing_bounds: (i32, i32, i32, i32), // left x, right x, top y, bottom y
}

impl App {
    pub fn new() -> App {
        App { player_pos: (0, 0), drawing: false, map: [['F'; 32]; 32], viewing_bounds: (0, 0, 0, 0) }
    }
}

impl Game for App {
    fn start(&mut self, term: &mut Term3D) {

    }

    fn update(&mut self, term: &mut Term3D, delta: f32, key: Option<Input>) {
        if let Some(input) = key {
            match input {
                Input::KeyRight => self.player_pos.0 = min(WORLD_WIDTH as i32, self.player_pos.0 + 1), // Go right
                Input::KeyLeft => self.player_pos.0 = max(0, self.player_pos.0 - 1), // Go left
                Input::KeyUp => self.player_pos.1 = max(0, self.player_pos.1 - 1), // Go up
                Input::KeyDown => self.player_pos.1 = min(WORLD_HEIGHT as i32, self.player_pos.1 + 1), // Go down
                Input::Character(' ') => self.drawing = !self.drawing,
                //e => term.log(&format!("{:?}", e), Color::White),
                _ => {}
            }
        }

        // Draw the player
        term3d::core::draw_cell(&mut term.backend, if self.drawing { 'O' } else { 'X' }, self.player_pos.0 - self.viewing_bounds.0, self.player_pos.1 - self.viewing_bounds.2);
    }
}

fn main() {
    let mut game = App::new();
    let mut term3d = Term3D::new();
    term3d.run(&mut game);
}
