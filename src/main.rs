extern crate easycurses;

use easycurses::*;

use std::thread::sleep;
use std::time::{Duration, Instant};

fn draw_cell(e: &mut EasyCurses, c: char, x: i32, y: i32) {
    e.move_xy(x, y);
    e.print_char(c);
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

    for x in x0..x1 {
        draw_cell(e, '#', x, y);
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

    for y in y0..y1 {
        draw_cell(e, '#', x, y);
        if d > 0 {
            x += xi;
            d -= 2 * dy;
        }
        d += 2 * dx;
    }
}

fn draw_line(e: &mut EasyCurses, x0: i32, y0: i32, x1: i32, y1: i32) {
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            draw_line_low(e, x1, y1, x0, y0);
        } else {
            draw_line_low(e, x0, y0, x1, y1);
        }
    } else {
        if y0 > y1 {
            draw_line_high(e, x1, y1, x0, y0);
        } else {
            draw_line_high(e, x0, y0, x1, y1);
        }
    }
}

fn rotate_2d(pos: (f32, f32), rad: f32) -> (f32, f32) {
    let (x, y) = pos;
    let (s, c) = (rad.sin(), rad.cos());
    (x * c - y * s, y * c + x * s)
}

struct Camera {
    pub pos: (f32, f32, f32),
    pub rot: (f32, f32),
}

impl Camera {
    pub fn new(pos: (f32, f32, f32), rot: (f32, f32)) -> Camera {
        Camera { pos, rot }
    }

    pub fn update(&mut self, _: &mut EasyCurses, delta: f32, key: Option<Input>) {
        let s = delta * 10.;

        if let Some(input) = key {
            match input {
                Input::Character('q') => self.pos.1 += s,
                Input::Character('e') => self.pos.1 -= s,

                Input::Character('w')
                | Input::Character('a')
                | Input::Character('s')
                | Input::Character('d') => {
                    let (x, y) = (s * self.rot.1.sin(), s * self.rot.1.cos());
                    match input {
                        Input::Character('w') => {
                            self.pos.0 += x;
                            self.pos.2 += y;
                        }
                        Input::Character('s') => {
                            self.pos.0 -= x;
                            self.pos.2 -= y;
                        }
                        Input::Character('a') => {
                            self.pos.0 -= y;
                            self.pos.2 += x;
                        }
                        Input::Character('d') => {
                            self.pos.0 += y;
                            self.pos.2 -= x;
                        }
                        _ => unreachable!(),
                    }
                }

                Input::KeyUp => self.rot.0 += s,
                Input::KeyDown => self.rot.0 -= s,
                Input::KeyLeft => self.rot.1 -= s,
                Input::KeyRight => self.rot.1 += s,

                _ => {}
            }
            println!("Input: {:?}", input);
        }
    }
}

fn main() {
    let mut easy = EasyCurses::initialize_system().unwrap();
    easy.set_input_mode(InputMode::Character);
    easy.set_input_timeout(TimeoutMode::Immediate);
    easy.set_cursor_visibility(CursorVisibility::Invisible);
    easy.set_keypad_enabled(true);
    easy.set_echo(false);

    let frame_target_duration = Duration::new(1, 0).checked_div(60).unwrap();

    let verts = [
        (-1., -1., -1.),
        (1., -1., -1.),
        (1., 1., -1.),
        (-1., 1., -1.),
        (-1., -1., 1.),
        (1., -1., 1.),
        (1., 1., 1.),
        (-1., 1., 1.),
    ];
    let edges = [
        (0., 1.),
        (1., 2.),
        (2., 3.),
        (3., 0.),
        (4., 5.),
        (5., 6.),
        (6., 7.),
        (7., 4.),
        (0., 4.),
        (1., 5.),
        (2., 6.),
        (3., 7.),
    ];

    let (mut h, mut w) = easy.get_row_col_count();
    let (mut cx, mut cy) = (w as f32 / 2., h as f32 / 2.);

    let mut delta_time: f32 = 0.;

    let mut cam = Camera::new((0., 0., -5.), (0., 0.));

    loop {
        let top_of_loop = Instant::now();

        let key = easy.get_input();
        if key == Some(Input::Character('\u{1b}')) {
            break;
        } else if key == Some(Input::KeyResize) {
            let (height, width) = easy.get_row_col_count();
            w = width;
            h = height;
            cx = w as f32 / 2.;
            cy = h as f32 / 2.;
        } else {
            cam.update(&mut easy, delta_time, key);
        }

        let after_updates = Instant::now();

        // clear screen
        for x in 0..w {
            for y in 0..h {
                draw_cell(&mut easy, ' ', x, y);
            }
        }

        for edge in &edges {
            let mut points: Vec<(f32, f32)> = Vec::new();
            for (x, y, z) in &[verts[edge.0 as usize], verts[edge.1 as usize]] {
                let x = x - cam.pos.0;
                let y = y - cam.pos.1;
                let z = z - cam.pos.2;

                let (mut x, z) = rotate_2d((x, z), cam.rot.1);
                let (mut y, z) = rotate_2d((y, z), cam.rot.0);

                let f = cx / z;
                x *= f;
                y *= f;
                points.push((cx + x, cy + y));
            }
            draw_line(
                &mut easy,
                points[0].0 as i32,
                points[0].1 as i32,
                points[1].0 as i32,
                points[1].1 as i32,
            );
        }

        let elapsed_this_frame = top_of_loop.elapsed();
        if let Some(frame_remaining) = frame_target_duration.checked_sub(elapsed_this_frame) {
            sleep(frame_remaining);
        }

        easy.refresh();

        let elapsed_after_updates = after_updates.elapsed();
        delta_time = (elapsed_after_updates.as_secs() as f32)
            + ((elapsed_after_updates.subsec_nanos() as f32) / 1000000000.0);
    }
}
