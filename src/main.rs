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
        easy.print_char('A');
    }
}

fn main() {
    let mut easy = EasyCurses::initialize_system().unwrap();
    easy.set_echo(false);
    
    draw_line(&mut easy, (0, 0), (7, 7));

    //loop {
        easy.refresh();
    //}

    easy.get_input();
}

