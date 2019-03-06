use term3d::*;

struct App {
    player_pos: (i32, i32),
    map: [[char; 32]; 32],
    viewing_bounds: (i32, i32, i32, i32), // min x offset, max x offset, min y offset, max y offset
}

impl App {
    pub fn new() -> App {
        App { player_pos: (0, 0), map: [['F'; 32]; 32], viewing_bounds: (0, 0, 0, 0) }
    }
}

impl Game for App {
    fn start(&mut self, term: &mut Term3D) {

    }

    fn update(&mut self, term: &mut Term3D, delta: f32, key: Option<Input>) {
        // Draw the player
        term3d::core::draw_cell(&mut term.backend, 'X', self.player_pos.0 + self.viewing_bounds.0, self.player_pos.1);
    }
}

fn main() {
    let mut game = App::new();
    let mut term3d = Term3D::new();
    term3d.run(&mut game);
}
