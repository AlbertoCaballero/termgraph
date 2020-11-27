mod draw;
use std::{thread, time};

fn main() {
    let mut cords = (0, 0);
    let mut width = 32;
    let mut height = 32;
    let mut n = 0;

    let ten_millis = time::Duration::from_millis(100);

    loop {
        let mut screen = String::new();
        if let Some((w, h)) = term_size::dimensions() {
            width = w;
            height = h;
            // Fill the screen
            for _x in 0..h - 1 {
                for _y in 0..w - 1 {
                    screen.push(' ');
                }
                screen.push(' ');
            }
            // Update and Draw coordinate lines
            cords = circle(n);
            screen = draw::coordinates(cords.0, cords.1, screen, w);
        } else {
            println!("Unable to get size!");
        }
        n += 1;
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "I: {} | Screen.len: {} | Width: {} | Height: {} | Coordinate X: {} | Coordinate Y: {}",
            n,
            screen.len(),
            width,
            height,
            cords.0,
            cords.1
        );
        print!("{}", screen);
        thread::sleep(ten_millis);
    }
}

fn circle(n: usize) -> (usize, usize) {
    (n % 45, n % 45)
}
