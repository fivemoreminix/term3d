use term3d::*;

struct App;

impl Game for App {
    fn start(&mut self, term: &mut Term3D) {
        // Initialize camera
        term.cam.pos = (6., -2., -10.);
        term.cam.rot = (0.15, -0.5);
        
        // Spawn a cube with a position and rotation
    }

    fn update(&mut self, term: &mut Term3D, delta: f32) {
        // Check for input
    }
}

fn main() {
    let mut term3d = Term3D::new();
    term3d.run(&mut App);
}
