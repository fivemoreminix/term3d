use term3d::*;

const COLORS: &[Color] = &[
    Color::Red,   // face 1
    Color::Green, // face 2
    Color::Blue,  // face 3, etc.
    Color::Yellow,
    Color::White,
    Color::Magenta,
];

struct App;

impl Game for App {
    fn start(&mut self, term: &mut Term3D) {
        // Initialize camera
        term.cam.transform = Transform {
            pos: (6., -2., -10.),
            rot: (0.15, -0.5),
        };

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

    fn update(&mut self, term: &mut Term3D, delta: f32, key: Option<Input>) {
        if let Some(input) = key {
            let s = delta * 10.;
            let cam = &mut term.cam;

            match input {
                Input::Character('q') => cam.transform.pos.1 += s, // Go down
                Input::Character('e') => cam.transform.pos.1 -= s, // Go up

                Input::Character('w')
                | Input::Character('a')
                | Input::Character('s')
                | Input::Character('d') => {
                    let (x, y) = (s * cam.transform.rot.1.sin(), s * cam.transform.rot.1.cos());
                    match input {
                        Input::Character('w') => { // Forward
                            cam.transform.pos.0 += x;
                            cam.transform.pos.2 += y;
                        }
                        Input::Character('s') => { // Backward 
                            cam.transform.pos.0 -= x;
                            cam.transform.pos.2 -= y;
                        }
                        Input::Character('a') => { // Left
                            cam.transform.pos.0 -= y;
                            cam.transform.pos.2 += x;
                        }
                        Input::Character('d') => { // Right
                            cam.transform.pos.0 += y;
                            cam.transform.pos.2 -= x;
                        }
                        _ => unreachable!(),
                    }
                }

                // The following inputs are for the arrow keys,
                // which in this example control looking around.
                Input::KeyUp => cam.transform.rot.0 -= s,
                Input::KeyDown => cam.transform.rot.0 += s,
                Input::KeyLeft => cam.transform.rot.1 -= s,
                Input::KeyRight => cam.transform.rot.1 += s,

                _ => {}
            }
        }
    }
}

fn main() {
    let mut term3d = Term3D::new();
    term3d.run(&mut App);
}
