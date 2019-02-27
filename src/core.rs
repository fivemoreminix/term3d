use easycurses::{EasyCurses, ColorPair, InputMode, CursorVisibility, TimeoutMode};

use crate::glm::IVec2;

use std::cmp::{max, min};

pub fn perp_ivec2(vec: &IVec2) -> IVec2 {
    IVec2::new(vec.y, -vec.x)
}

pub fn rotate_2d(pos: (f32, f32), rad: f32) -> (f32, f32) {
    let (x, y) = pos;
    let (s, c) = (rad.sin(), rad.cos());
    (x * c - y * s, y * c + x * s)
}

pub trait Backend {
    /// Create and initialize the backend.
    /// 
    /// A correctly initialized backend should have no visible cursor.
    fn new() -> Self;
    /// Print a single character with the given color to the x and y position on the terminal,
    /// in the fastest possible way. This function will be called thousands, to hundreds of thousands
    /// of times in a single frame.
    fn draw(&mut self, c: char, x: i32, y: i32);
    /// Set the color to draw future characters with.
    fn set_color(&mut self, color: ColorPair);
    /// Clear the terminal of all characters.
    fn clear(&mut self);
    /// Return width and height (in character cells) of the terminal window.
    fn get_dimensions(&self) -> (i32, i32);
}

struct CursesBackend(EasyCurses);

impl Backend for CursesBackend {
    fn new() -> Self {
        let mut curses = EasyCurses::initialize_system().unwrap();

        curses.set_input_mode(InputMode::Character);
        curses.set_input_timeout(TimeoutMode::Immediate);
        curses.set_cursor_visibility(CursorVisibility::Invisible);
        curses.set_keypad_enabled(true);
        curses.set_echo(false);

        CursesBackend(curses)
    }

    fn draw(&mut self, c: char, x: i32, y: i32) {
        self.0.move_rc(y, x);
        self.0.print_char(c);
    }
    
    fn set_color(&mut self, color: ColorPair) {
        self.0.set_color_pair(color);
    }

    fn clear(&mut self) {
        let (h, w) = self.0.get_row_col_count();
        self.0.set_color_pair(ColorPair::default());
        for x in 0..w {
            for y in 0..h {
                self.draw(' ', x, y);
            }
        }
    }

    fn get_dimensions(&self) -> (i32, i32) {
        let (y, x) = self.0.get_row_col_count();
        (x, y)
    }
}

pub fn draw_cell(e: &mut EasyCurses, c: char, x: i32, y: i32) {
    // Top left is origin
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

pub fn draw_line(e: &mut EasyCurses, x0: i32, y0: i32, x1: i32, y1: i32) {
    if x0 == x1 {
        for y in y0..=y1 {
            draw_cell(e, '|', x0, y);
        }
    } else if y0 == y1 {
        for x in x0..=x1 {
            draw_cell(e, '-', x, y0);
        }
    } else {
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
        let (minx, maxx, miny, maxy) = tri_bounding_box(v1, v2, v3);
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
                draw_cell(e, '#', x, y);
            }
        }
    }
}

pub fn draw_quad(e: &mut EasyCurses, color: ColorPair, a: IVec2, b: IVec2, c: IVec2, d: IVec2) {
    draw_tri(e, color, a, b, c);
    draw_tri(e, color, a, d, c);
}
