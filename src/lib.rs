#![feature(vec_remove_item)]

pub use easycurses::*;
pub use nalgebra_glm as glm;

use ordered_float::NotNan;

use glm::IVec2;

use std::cmp::{max, min};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn perp_ivec2(vec: &IVec2) -> IVec2 {
    IVec2::new(vec.y, -vec.x)
}

pub fn rotate_2d(pos: (f32, f32), rad: f32) -> (f32, f32) {
    let (x, y) = pos;
    let (s, c) = (rad.sin(), rad.cos());
    (x * c - y * s, y * c + x * s)
}

pub struct Camera {
    pub transform: Transform,
}

impl Camera {
    pub fn new(pos: (f32, f32, f32), rot: (f32, f32)) -> Self {
        Self {
            transform: Transform { pos, rot },
        }
    }
}

pub struct Mesh {
    pub verts: Vec<[f32; 3]>,
    pub faces: Vec<([f32; 4], Option<Color>)>,
}

impl Mesh {
    #[inline]
    pub fn cube() -> Self {
        Self {
            verts: vec![
                [-1., -1., -1.],
                [1., -1., -1.],
                [1., 1., -1.],
                [-1., 1., -1.],
                [-1., -1., 1.],
                [1., -1., 1.],
                [1., 1., 1.],
                [-1., 1., 1.],
            ],
            faces: vec![
                ([0., 1., 2., 3.], None),
                ([4., 5., 6., 7.], None),
                ([0., 1., 5., 4.], None),
                ([2., 3., 7., 6.], None),
                ([0., 3., 7., 4.], None),
                ([1., 2., 6., 5.], None),
            ],
        }
    }
}

pub struct Transform {
    pub pos: (f32, f32, f32),
    pub rot: (f32, f32),
}

impl Transform {
    pub fn new() -> Self {
        Self {
            pos: (0., 0., 0.),
            rot: (0., 0.),
        }
    }
}

pub struct Object {
    pub transform: Transform,
    pub mesh: Mesh,
}

impl Object {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            transform: Transform::new(),
            mesh,
        }
    }
}

pub trait Game {
    fn start(&mut self, term: &mut Term3D);
    fn update(&mut self, term: &mut Term3D, delta: f32, key: Option<Input>);
}

pub struct Term3D {
    pub backend: EasyCurses,
    pub cam: Camera,
    pub objects: Vec<Object>,
    pub log: Vec<(String, Color, Duration)>, // Lines of text being drawn
}

impl Term3D {
    pub fn new() -> Self {
        Self {
            backend: EasyCurses::initialize_system().unwrap(),
            cam: Camera::new((0., 0., 0.), (0., 0.)),
            objects: Vec::new(),
            log: Vec::new(),
        }
    }

    pub fn run<T: Game>(&mut self, game: &mut T) {
        self.backend.set_input_mode(InputMode::Character);
        self.backend.set_input_timeout(TimeoutMode::Immediate);
        self.backend
            .set_cursor_visibility(CursorVisibility::Invisible);
        self.backend.set_keypad_enabled(true);
        self.backend.set_echo(false);

        let frame_target_duration = Duration::new(1, 0).checked_div(60).unwrap();

        let (mut h, mut w) = self.backend.get_row_col_count();
        let (mut cx, mut cy) = (w as f32 / 2., h as f32 / 2.);

        let mut delta_time: f32 = 0.;

        // Initialize game
        game.start(self);

        self.log("Game started!", Color::Green);
        self.log("", Color::Red);

        loop {
            let top_of_loop = Instant::now();

            let key = self.backend.get_input();
            if key == Some(Input::Character('\u{1b}')) {
                break;
            } else if key == Some(Input::KeyResize) {
                self.backend.resize(0, 0);
                let (height, width) = self.backend.get_row_col_count();
                w = width;
                h = height;
                cx = w as f32 / 2.;
                cy = h as f32 / 2.;
            }

            game.update(self, delta_time, key);

            self.log[1].0 = format!("{}", self.log[1].2.as_secs());

            //let after_updates = Instant::now();

            // clear screen
            self.backend.set_color_pair(ColorPair::default());
            for x in 0..w {
                for y in 0..h {
                    Self::draw_cell(&mut self.backend, ' ', x, y);
                }
            }

            let mut face_list = Vec::<([IVec2; 4], Option<Color>)>::new(); // All faces that will be rendered onto the screen
                                                                           //let mut face_color = Vec::<Color>::new(); // Colors in the same length and order as face_list
            let mut depth = Vec::<f32>::new(); // Face's distances from the camera

            for obj in &self.objects {
                // Vertices after mutation by camera position and rotation,
                // and object position offset.
                let mut vert_list = Vec::<[f32; 3]>::new();
                // Position of the vertices in vert_list as screen
                // coordinates.
                let mut screen_coords = Vec::<IVec2>::new();

                for vert in &obj.mesh.verts {
                    let (x, y, z) = (
                        vert[0] - self.cam.transform.pos.0,
                        vert[1] / 2. - self.cam.transform.pos.1,
                        vert[2] - self.cam.transform.pos.2,
                    );
                    let (mut x, z) = rotate_2d((x, z), self.cam.transform.rot.1);
                    let (mut y, z) = rotate_2d((y, z), self.cam.transform.rot.0);
                    vert_list.push([x, y, z]);

                    let f = 200. / z;
                    x *= f;
                    y *= f;
                    screen_coords.push(IVec2::new((cx + x) as i32, (cy + y) as i32));
                }

                for i in 0..obj.mesh.faces.len() {
                    let face = obj.mesh.faces[i];

                    let mut on_screen = false;
                    for &i in &face.0 {
                        let p = screen_coords[i as usize];
                        // If any of the face's corners are within view
                        if /*vert_list[i as usize][2] > 0. &&*/ (p.x >= 0 && p.x <= w) || (p.y >= 0 && p.y <= h)
                        {
                            on_screen = true;
                            break; // Break from the iteration
                        }
                    }

                    if on_screen {
                        face_list.push((
                            //face.0.iter().map(|&v| screen_coords[v as usize]).collect(),
                            [
                                screen_coords[face.0[0] as usize],
                                screen_coords[face.0[1] as usize],
                                screen_coords[face.0[2] as usize],
                                screen_coords[face.0[3] as usize],
                            ],
                            face.1,
                        ));
                        //face_color.push(COLORS[i]);

                        // depth += [sum(sum(vert_list[j][k] for j in face)**2 for k in range(3))]
                        depth.push(
                            (0..3)
                                .map(|k| {
                                    face.0
                                        .iter()
                                        .map(|&j| vert_list[j as usize][k as usize])
                                        .sum::<f32>()
                                        .powi(2)
                                })
                                .sum::<f32>(),
                        );
                    }
                }
            }

            let mut order = (0..face_list.len()).collect::<Vec<usize>>();
            order.sort_by_key(|&k| NotNan::new(depth[k]).unwrap());
            order.reverse();

            for i in order {
                Self::draw_quad(
                    &mut self.backend,
                    ColorPair::new(
                        match face_list[i].1 {
                            Some(c) => c,
                            None => Color::White,
                        },
                        Color::Black,
                    ),
                    face_list[i].0[0],
                    face_list[i].0[1],
                    face_list[i].0[2],
                    face_list[i].0[3],
                );
            }

            if !self.log.is_empty() {
                let mut to_be_removed = Vec::<usize>::new();

                for i in 0..self.log.len() {
                    // Draw log text
                    self.backend.move_rc(i as i32, 0);
                    self.backend.set_color_pair(ColorPair::new(self.log[i].1, Color::Black));
                    self.backend.print(&self.log[i].0);

                    match self.log[i].2.checked_sub(Duration::from_millis((delta_time * 1000.) as u64)) {
                        None => to_be_removed.push(i),
                        Some(v) => self.log[i].2 = v,
                    }
                }

                if !to_be_removed.is_empty() {
                    let mut offset = 0;
                    for index in to_be_removed {
                        // This only works because the items in to_be_removed are added
                        // in the same order as 0..self.log.len() (they are sorted)
                        self.log.remove(index - offset);
                        offset += 1;
                    }
                }
            }

            let elapsed_this_frame = top_of_loop.elapsed();
            // Sleep the remainder of the target frame rate time
            if let Some(frame_remaining) = frame_target_duration.checked_sub(elapsed_this_frame) {
                sleep(frame_remaining);
            }

            self.backend.refresh();

            //let elapsed_after_updates = after_updates.elapsed();
            //delta_time = (elapsed_after_updates.as_secs() as f32)
            //    + ((elapsed_after_updates.subsec_nanos() as f32) / 1000000000.0);
            delta_time = elapsed_this_frame.subsec_nanos() as f32 / 1000000000.0;
        }
    }

    pub fn draw_cell(e: &mut EasyCurses, c: char, x: i32, y: i32) {
        // Top left is origin
        e.move_rc(y, x);
        e.print_char(c);
    }

    pub fn log(&mut self, text: &str, color: Color) {
        self.log.insert(0, (text.to_owned(), color, Duration::from_secs(10)));
        if self.log.len() > self.backend.get_row_col_count().0 as usize {
            self.log.pop();
        }
    }

    fn draw_line_low(e: &mut EasyCurses, x0: i32, y0: i32, x1: i32, y1: i32) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut d = 2 * dy - dx;
        let mut y = y0;

        e.set_color_pair(ColorPair::default());
        for x in x0..x1 {
            Term3D::draw_cell(e, '#', x, y);
            if d > 0 {
                y += yi;
                d -= 2 * dx;
            }
            d += 2 * dy;
        }
    }

    fn draw_line_high(e: &mut EasyCurses, x0: i32, y0: i32, x1: i32, y1: i32) {
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut d = 2 * dx - dy;
        let mut x = x0;

        e.set_color_pair(ColorPair::default());
        for y in y0..y1 {
            Term3D::draw_cell(e, '#', x, y);
            if d > 0 {
                x += xi;
                d -= 2 * dy;
            }
            d += 2 * dx;
        }
    }

    pub fn draw_line(e: &mut EasyCurses, x0: i32, y0: i32, x1: i32, y1: i32) {
        if x0 == x1 {
            for y in y0..=y1 {
                Self::draw_cell(e, '|', x0, y);
            }
        } else if y0 == y1 {
            for x in x0..=x1 {
                Self::draw_cell(e, '-', x, y0);
            }
        } else {
            if (y1 - y0).abs() < (x1 - x0).abs() {
                if x0 > x1 {
                    Self::draw_line_low(e, x1, y1, x0, y0);
                } else {
                    Self::draw_line_low(e, x0, y0, x1, y1);
                }
            } else {
                if y0 > y1 {
                    Self::draw_line_high(e, x1, y1, x0, y0);
                } else {
                    Self::draw_line_high(e, x0, y0, x1, y1);
                }
            }
        }
    }

    /// # Returns
    /// (minimum x, maximum x, minimum y, maximum y)
    pub fn tri_bounding_box(v1: IVec2, v2: IVec2, v3: IVec2) -> (i32, i32, i32, i32) {
        let mut min_x = v1.x;
        let mut max_x = v1.x;
        let mut min_y = v1.y;
        let mut max_y = v1.y;

        for vec in &[v2, v3] {
            if vec.x < min_x {
                min_x = vec.x;
            } else if vec.x > max_x {
                max_x = vec.x;
            }

            if vec.y < min_y {
                min_y = vec.y;
            } else if vec.y > max_y {
                max_y = vec.y;
            }
        }

        (min_x, max_x, min_y, max_y)
    }

    pub fn draw_tri(e: &mut EasyCurses, color: ColorPair, v1: IVec2, v2: IVec2, v3: IVec2) {
        // calculate triangle bounding box
        let (minx, maxx, miny, maxy) = {
            let (minx, maxx, miny, maxy) = Self::tri_bounding_box(v1, v2, v3);
            // Clip box against render target bounds
            let (mut emax_y, mut emax_x) = e.get_row_col_count();
            emax_y -= 1;
            emax_x -= 1;
            (
                min(emax_x, max(0, minx)),
                min(emax_x, max(0, maxx)),
                min(emax_y, max(0, miny)),
                min(emax_y, max(0, maxy)),
            )
        };

        let vs1 = IVec2::new(v2.x - v1.x, v2.y - v1.y);
        let vs2 = IVec2::new(v3.x - v1.x, v3.y - v1.y);

        e.set_color_pair(color);
        for x in minx..=maxx {
            for y in miny..=maxy {
                let q = IVec2::new(x - v1.x, y - v1.y);

                let perp_dot_product_vs1_vs2 = perp_ivec2(&vs1).dot(&vs2) as f32;
                let s = perp_ivec2(&q).dot(&vs2) as f32 / perp_dot_product_vs1_vs2;
                let t = perp_ivec2(&vs1).dot(&q) as f32 / perp_dot_product_vs1_vs2;
                //let s = q.perp_dot_product(&vs2) / vs1.perp_dot_product(&vs2);
                //let t = vs1.perp_dot_product(&q) / vs1.perp_dot_product(&vs2);

                if (s >= 0.) && (t >= 0.) && (s + t <= 1.) {
                    Self::draw_cell(e, '#', x, y);
                }
            }
        }
    }

    pub fn draw_quad(e: &mut EasyCurses, color: ColorPair, a: IVec2, b: IVec2, c: IVec2, d: IVec2) {
        Self::draw_tri(e, color, a, b, c);
        Self::draw_tri(e, color, a, d, c);
    }
}
