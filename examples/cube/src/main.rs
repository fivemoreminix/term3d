use term3d::*;

const COLORS: &[Color] = &[
    Color::Red, // face 1
    Color::Green, // face 2
    Color::Blue, // face 3, etc.
    Color::Yellow,
    Color::White,
    Color::Magenta,
];

struct App;

impl Game for App {
    fn start(&mut self, term: &mut Term3D) {
        // Initialize camera
        term.cam.transform = Transform { pos: (6., -2., -10.), rot: (0.15, -0.5) };

        // Create a cube Mesh with vertex and face data
        let mut cube = Mesh::cube();
        // Color the faces different colors
        for i in 0..cube.faces.len() {
            cube.faces[i].1 = Some(COLORS[i]);
        }
        // Create an object with the cube mesh
        let obj = Object::new(cube);
        // Add the object to the scene
        term.objects.push(obj);
    }

    fn update(&mut self, term: &mut Term3D, delta: f32) {
        // Check for input
    }
}

fn main() {
    let mut term3d = Term3D::new();
    term3d.run(&mut App);
}
