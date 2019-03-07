use term3d::*;

use std::cmp::{min, max};

const WORLD_WIDTH: usize = 32;
const WORLD_HEIGHT: usize = 32;
const PLAYER_SPEED: f32 = 40.;

struct App {
    player_pos: (f32, f32),
    drawing: bool,
    direction: (i32, i32), // direction of movement: x, y; -1, 1; 0, 0; etc.
    map: [[char; WORLD_HEIGHT]; WORLD_WIDTH],
    viewing_bounds: (i32, i32, i32, i32), // left x, right x, top y, bottom y
}

impl App {
    pub fn new() -> App {
        App { player_pos: (0., 0.), drawing: false, direction: (0, 0), map: [['F'; 32]; 32], viewing_bounds: (0, 0, 0, 0) }
    }
}

impl Game for App {
    fn start(&mut self, term: &mut Term3D) {

    }

    fn update(&mut self, term: &mut Term3D, delta: f32, key: Option<Input>) {
        // Update direction
        if let Some(input) = key {
            match input {
                Input::KeyRight => self.direction = (1, 0), // Go right
                Input::KeyLeft => self.direction = (-1, 0), // Go left
                Input::KeyUp => self.direction = (0, -1), // Go up
                Input::KeyDown => self.direction = (0, 1), // Go down
                Input::Character(' ') => self.drawing = !self.drawing,
                Input::Character('p') => self.direction = (0, 0), // Stop player's movement
                //e => term.log(&format!("{:?}", e), Color::White),
                _ => {}
            }
        }

        // Update position based on direction
        //self.player_pos.0 = max(0., min(WORLD_WIDTH as f32, self.player_pos.0 + self.direction.0 as f32 * delta));
        self.player_pos.0 += self.direction.0 as f32 * delta * PLAYER_SPEED;
        //self.player_pos.1 = max(0., min(WORLD_HEIGHT as f32, self.player_pos.1 + self.direction.1 as f32 * delta));
        self.player_pos.1 += self.direction.1 as f32 * delta * PLAYER_SPEED;
        // self.player_pos.0 += self.direction.0;
        // self.player_pos.1 += self.direction.1;

        // Draw the player
        term3d::core::draw_cell(&mut term.backend, if self.drawing { 'O' } else { 'X' }, self.player_pos.0 as i32 - self.viewing_bounds.0, self.player_pos.1 as i32 - self.viewing_bounds.2);
    }
}

fn main() {
    let mut game = App::new();
    let mut term3d = Term3D::new();
    term3d.run(&mut game);
}
