use term3d::*;

use std::collections::HashMap;
use std::cmp::{min, max};

const WORLD_WIDTH: usize = 32;
const WORLD_HEIGHT: usize = 32;
const PLAYER_SPEED: f32 = 40.;

struct Faction {
    id: u32,
    color: Color,
}

struct App {
    player_pos: (i32, i32),
    player_fac_id: u32,
    // drawing: bool,
    // direction: (i32, i32), // direction of movement: x, y; -1, 1; 0, 0; etc.
    map: [[Option<Color>; WORLD_HEIGHT]; WORLD_WIDTH],
    factions: HashMap<u32, Faction>, // id, faction value
    viewing_bounds: (i32, i32, i32, i32), // left x, right x, top y, bottom y
}

impl App {
    pub fn new() -> App {
        App { player_pos: (0, 0), player_fac_id: 1, /*drawing: false, direction: (0, 0),*/ map: [[None; 32]; 32], factions: {
            let mut map = HashMap::new();
            map.insert(1, Faction { id: 1, color: Color::Yellow });
            map
        }, viewing_bounds: (0, 0, 0, 0) }
    }

    fn map_create_spawn(&mut self, center: (i32, i32), faction: Color) {
        for x in center.0-1..=center.0+1 {
            for y in center.1-1..=center.1+1 {
                self.map[x as usize][y as usize] = Some(faction);
            }
        }
    }
}

impl Game for App {
    fn start(&mut self, term: &mut Term3D) {
        let spawn_origin = (3i32, 5i32);
        self.player_pos = (spawn_origin.0 as i32, spawn_origin.1 as i32);
        self.map_create_spawn(spawn_origin, self.factions[&self.player_fac_id].color);
    }

    fn update(&mut self, term: &mut Term3D, delta: f32, key: Option<Input>) {
        // Update direction
        if let Some(input) = key {
            match input {
                Input::KeyRight => self.player_pos.0 = min(WORLD_WIDTH as i32, self.player_pos.0 + 1), // Go right
                Input::KeyLeft => self.player_pos.0 = max(0, self.player_pos.0 - 1), // Go left
                Input::KeyUp => self.player_pos.1 = max(0, self.player_pos.1 - 1), // Go up
                Input::KeyDown => self.player_pos.1 = min(WORLD_HEIGHT as i32, self.player_pos.1 + 1), // Go down
                //Input::Character(' ') => self.drawing = !self.drawing,
                //Input::Character('p') => self.direction = (0, 0), // Stop player's movement
                //e => term.log(&format!("{:?}", e), Color::White),
                _ => {}
            }
        }

        // Update viewing bounds
        self.viewing_bounds = { // x tiles 2-6 is equal to (1, 5, y, y) counting starts at zero
            let (h, w) = term.backend.get_row_col_count();
            (self.viewing_bounds.0, min(WORLD_WIDTH as i32 - 1, w - 1), self.viewing_bounds.2, min(WORLD_HEIGHT as i32 - 1, h - 1))
        };

        // Draw the map (index only map tiles within view bounds)
        for x in self.viewing_bounds.0..self.viewing_bounds.1 {
            for y in self.viewing_bounds.2..self.viewing_bounds.3 {
                if x == 0 {
                    term.backend.set_color_pair(ColorPair::new(Color::Red, Color::Black));
                    term3d::core::draw_cell(&mut term.backend, '|', x, y + 2);
                } else if let Some(c) = self.map[x as usize][y as usize] {
                    term.backend.set_color_pair(ColorPair::new(Color::White, c));
                    term3d::core::draw_cell(&mut term.backend, ' ', x, y + 2);
                }
            }
        }

        // Draw the player
        term.set_color(ColorPair::default());
        term.draw('X', self.player_pos.0 as i32 - self.viewing_bounds.0, self.player_pos.1 as i32 - self.viewing_bounds.2 + 2);

        // Print status
        //term.say(&format!("id: {}", self.factions[&self.player_fac_id].id), 0, 0);
    }
}

fn main() {
    let mut game = App::new();
    let mut term3d = Term3D::new();
    term3d.run(&mut game);
}
