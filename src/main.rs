extern crate easycurses;

use easycurses::Color::*;
use easycurses::*;

fn draw_cell(e: &mut EasyCurses, c:char,x:i32,y:i32) {
    e.move_xy(x, y);
    e.print_char(c);
}

fn draw_line_low(e: &mut EasyCurses, x0:i32,y0:i32,x1:i32,y1:i32) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut D = 2*dy - dx;
    let mut y = y0;

    for x in x0..x1 {
        draw_cell(e, '#', x, y);
        if D > 0 {
            y += yi;
            D -= 2*dx;
        }
        D += 2*dy;
    }
}

fn draw_line_high(e: &mut EasyCurses, x0:i32,y0:i32,x1:i32,y1:i32) {
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut D = 2*dx - dy;
    let mut x = x0;

    for y in y0..y1 {
        draw_cell(e, '#', x, y);
        if D > 0 {
            x += xi;
            D -= 2*dy;
        }
        D += 2*dx;
    }
}

fn draw_line(e: &mut EasyCurses, x0:i32,y0:i32,x1:i32,y1:i32) {
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

fn main() {
    let mut easy = EasyCurses::initialize_system().unwrap();
    easy.set_echo(false);

    let (h,w) = easy.get_row_col_count();
    let (cx,cy): (i32, i32) = (w/2,h/2);

    let verts = [(-1,-1,-1),(1,-1,-1),(1,1,-1),(-1,1,-1),(-1,-1,1),(1,-1,1),(1,1,1),(-1,1,1)];
    let edges = [(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(6,7),(7,4),(0,4),(1,5),(2,6),(3,7)];

    for (x,y,z) in &verts {
        let z = z + 5;
        let f = (w/2)/z;
        let x = x * f;
        let y = y * f;
        easy.move_xy(cx+x, cy+y);
        easy.print_char('%');
    }

    for edge in &edges {
        let mut points: Vec<(i32,i32)> = Vec::new();
        for (x,y,z) in &[verts[edge.0],verts[edge.1]] {
            let z = z + 5;
            let f = (w/2)/z;
            let x = x * f;
            let y = y * f;
            points.push((cx+x, cy+y));
        }
        draw_line(&mut easy, points[0].0, points[0].1, points[1].0, points[1].1);
    }

    //loop {
        easy.refresh();
    //}

    easy.get_input();
}

