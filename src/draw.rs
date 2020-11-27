pub fn square(x: usize, y: usize, width: usize, height: usize, screen: String, space: (usize, usize), fill: char) -> String {
    //! Draw Square Function
    //!! coordinates: (u32, u32) -> coordinate from where to start
    //!! width: u32, height: u32 -> width and height for the figure
    //!! screen: String -> the current screen to basically draw on top of it
    //!! space: (usize, usize) -> width and height of the screen
    //!! fill: char -> infill char for the figure
    //!!
    //!! return: String -> input screen with shape incerted

    // Create new screen String Array
    let mut new_screen = String::new();

    for (p, v) in screen.char_indices() {
        // p -> position in array
        // v -> value in position
        if p % width != 0 {
            // inside a line, after start
            if x == p % width {
                // if we are at right x
                new_screen.push(fill);
            } else {
                new_screen.push(v);
            }
        } else {
            // Left Edge of screen
            // Start of a new line
            new_screen.push(v);
        }
    }
    new_screen
}

pub fn coordinates(x: usize, y: usize, screen: String, width: usize) -> String {
    //! Draw Coordinate lines
    //!! x: usize -> X coordinate
    //!! y: usize -> Y coordinate
    //!! screen: String -> Screen to draw on top of
    //!!
    //!! return: String -> New Scree

    let mut new_screen = screen;
    new_screen = line_x(x, new_screen, width, '|');
    new_screen = line_y(y, new_screen, width, '-');
    new_screen
}

pub fn line_x(x: usize, screen: String, width: usize, fill: char) -> String {
    //! Draw a line in X
    let mut new_screen = String::new();
    for (p, v) in screen.char_indices() {
        if x == p % width { new_screen.push(fill) }
        else { new_screen.push(v) }
    }
    new_screen
}

pub fn line_y(y: usize, screen: String, width: usize, fill: char) -> String {
    //! Draw a line in Y
    let mut new_screen = String::new();
    let mut current_y = 0; 
    for (p, v) in screen.char_indices() {
        if p % width == 0 { current_y += 1 }
        if current_y == y { 
            new_screen.push(fill);
        } else { 
            new_screen.push(v);
        }
    }
    new_screen
}

