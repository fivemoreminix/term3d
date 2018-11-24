extern crate easycurses;

use easycurses::Color::*;
use easycurses::*;

fn draw_line(easy: &mut EasyCurses, pos1: (i32, i32), pos2: (i32, i32)) {
    let a = 2 * (pos2.1 - pos1.1); // a = 2 * change in Y
    let b = a - (2 * (pos2.0 - pos1.0)); // b = a - 2 * change in X
    let mut p = a - (pos2.0 - pos1.0); // p = a - change in X

    easy.move_xy(pos1.0, pos1.1);
    
    let mut y = pos1.1;
    for x in pos1.0..pos2.0 {
        if p < 0 {
            p += a;
        } else /*if p >= 0*/ {
            y += 1;
            p += b;
        }

        easy.move_xy(x, y);
        easy.print_char('#');
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
        draw_line(&mut easy, points[0], points[1]);
    }

    //loop {
        easy.refresh();
    //}

    easy.get_input();
}

