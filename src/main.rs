extern crate easycurses;
extern crate ordered_float;

use easycurses::*;

use ordered_float::NotNan;

mod prelude;

use crate::prelude::*;

use std::cmp::{max, min};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn draw_cell(e: &mut EasyCurses, c: char, x: i32, y: i32) {
    // e.move_xy(x, y);
    e.move_rc(y, x);
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

    e.set_color_pair(ColorPair::default());
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

    e.set_color_pair(ColorPair::default());
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

// fn draw_bottom_flat_tri(e: &mut EasyCurses, v1: IVec2, v2: IVec2, v3: IVec2) {
//     let invslope1 = (v2.x - v1.x) / (v2.y - v1.y + 1);
//     let invslope2 = (v3.x - v1.x) / (v3.y - v1.y);

//     let mut curx1 = v1.x;
//     let mut curx2 = v1.x;

//     for scanline_y in v1.y..v2.y {
//         draw_line(e, curx1, scanline_y, curx2, scanline_y);
//         curx1 += invslope1;
//         curx2 += invslope2;
//     }
// }

// fn draw_top_flat_tri(e: &mut EasyCurses, v1: IVec2, v2: IVec2, v3: IVec2) {
//     let invslope1 = (v3.x - v1.x) / (v3.y - v1.y);
//     let invslope2 = (v3.x - v2.x) / (v3.y - v2.y);

//     let mut curx1 = v3.x;
//     let mut curx2 = v3.x;

//     for scanline_y in (v1.y + 1..=v3.y).rev() {
//         draw_line(e, curx1, scanline_y, curx2, scanline_y);
//         curx1 -= invslope1;
//         curx2 -= invslope2;
//     }
// }

// fn draw_tri(e: &mut EasyCurses, v1: IVec2, v2: IVec2, v3: IVec2) {
//     // sort the three vertices by y-coordinate ascending so v1 is topmost vertice
//     let (mut y1, mut y2, mut y3);
//     {
//         let mut y = [v1.y, v2.y, v3.y];
//         y.sort();
//         y1 = y[0];
//         y2 = y[1];
//         y3 = y[2];
//     }

//     if y2 == y3 {
//         draw_bottom_flat_tri(e, v1, v2, v3);
//         //println!("bottom flat tri");
//     } else if y1 == y2 {
//         draw_top_flat_tri(e, v1, v2, v3);
//         //println!("top flat tri");
//     } else {
//         // split the triangle into a top-flat and bottom-flat
//         let v4 = IVec2::new(v1.x + ((v2.y - v1.y) / (v3.y - v1.y)) * (v3.x - v1.x), v2.y);
//         draw_bottom_flat_tri(e, v1, v2, v4);
//         draw_bottom_flat_tri(e, v2, v4, v3);
//         //println!("split into two tris");
//     }
// }

/// # Returns
/// (minimum x, maximum x, minimum y, maximum y)
fn tri_bounding_box(v1: IVec2, v2: IVec2, v3: IVec2) -> (i32, i32, i32, i32) {
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

fn draw_tri(e: &mut EasyCurses, color: ColorPair, v1: IVec2, v2: IVec2, v3: IVec2) {
    // calculate triangle bounding box
    let (minx, maxx, miny, maxy) = {
        let (minx, maxx, miny, maxy) = tri_bounding_box(v1, v2, v3);
        // Clip box against render target bounds
        let (emax_y, emax_x) = e.get_row_col_count();
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

            let s = q.perp_dot_product(&vs2) / vs1.perp_dot_product(&vs2);
            let t = vs1.perp_dot_product(&q) / vs1.perp_dot_product(&vs2);

            if (s >= 0.) && (t >= 0.) && (s + t <= 1.) {
                draw_cell(e, '#', x, y);
            }
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
            // println!("Input: {:?}", input);
        }
    }
}

const COLORS: &[Color] = &[
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::Yellow,
    Color::White,
    Color::Magenta,
];

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
    // let edges = [
    //     (0., 1.),
    //     (1., 2.),
    //     (2., 3.),
    //     (3., 0.),
    //     (4., 5.),
    //     (5., 6.),
    //     (6., 7.),
    //     (7., 4.),
    //     (0., 4.),
    //     (1., 5.),
    //     (2., 6.),
    //     (3., 7.),
    // ];
    let faces = [
        (0., 1., 2., 3.),
        (4., 5., 6., 7.),
        (0., 1., 5., 4.),
        (2., 3., 7., 6.),
        (0., 3., 7., 4.),
        (1., 2., 6., 5.),
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
            easy.resize(0, 0);
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
        easy.set_color_pair(ColorPair::default());
        for x in 0..w {
            for y in 0..h {
                draw_cell(&mut easy, ' ', x, y);
            }
        }

        let mut vert_list = Vec::new();
        let mut screen_coords = Vec::<IVec2>::new();

        for (x, y, z) in &verts {
            let (x, y, z) = (x - cam.pos.0, y - cam.pos.1, z - cam.pos.2);
            let (mut x, z) = rotate_2d((x, z), cam.rot.1);
            let (mut y, z) = rotate_2d((y, z), cam.rot.0);
            vert_list.push((x, y, z));

            let f = 200. / z;
            x *= f;
            y *= f;
            screen_coords.push(IVec2::new((cx + x) as i32, (cy + y) as i32));
        }

        let mut face_list = Vec::<Vec<IVec2>>::new();
        let mut face_color = Vec::<Color>::new();
        let mut depth = Vec::<f32>::new();

        for i in 0..faces.len() {
            let (a, b, c, d) = faces[i];
            let mut on_screen = false;
            for &i in &[a, b, c, d] {
                if vert_list[i as usize].2 > 0. {
                    on_screen = true;
                    break;
                }
            }

            if on_screen {
                face_list.push(
                    [a, b, c, d]
                        .iter()
                        .map(|&v| screen_coords[v as usize])
                        .collect(),
                );
                face_color.push(COLORS[i]);

                depth.push(
                    vert_list.iter().map(|v| v.0).sum::<f32>().powf(2.)
                        + vert_list.iter().map(|v| v.1).sum::<f32>().powf(2.)
                        + vert_list.iter().map(|v| v.2).sum::<f32>().powf(2.),
                );
            }
        }

        let mut order = (0..face_list.len()).collect::<Vec<usize>>();
        order.sort_by_key(|&k| NotNan::new(depth[k]).unwrap());

        for i in order {
            draw_tri(
                &mut easy,
                ColorPair::new(face_color[i], Color::Black),
                face_list[i][0],
                face_list[i][1],
                face_list[i][2],
            );
            draw_tri(
                &mut easy,
                ColorPair::new(face_color[i], Color::Black),
                face_list[i][0],
                face_list[i][3],
                face_list[i][2],
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
