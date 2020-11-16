mod draw;
use std::{thread, time};

fn main() {
    let mut cord_x = 30;
    let mut cord_y = 12;
    let mut width = 32;
    let mut height = 32;
    let mut n = 0;

    let ten_millis = time::Duration::from_millis(600);

    loop {
        let mut screen = String::new();
        if let Some((w, h)) = term_size::dimensions() {
            width = w;
            height = h;
            // Fill the screen
            for _x in 0..h-2 {
                for _y in 0..w-1 {
                    screen.push(' ');
                } screen.push(' ');
            }
            // Update and Draw coordinate lines
            cord_x += 1;
            cord_y += 1;
            screen = draw::coordinates(cord_x, cord_y, screen, w);
        } else {
            println!("Unable to get size!");
        }

        n += 1;
        print!("{}", screen);
        println!("I: {} | Screen.len: {} | Width: {} | Height: {} | Cord X: {} | Cord Y: {}",
                n, screen.len(), width, height, cord_x, cord_y);
        thread::sleep(ten_millis);
        print!("\x1B[2J\x1B[1;1H");
    }
}

