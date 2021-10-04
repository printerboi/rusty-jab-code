#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Color{
    BLACK,
    BLUE,
    GREEN,
    CYAN,
    RED,
    MAGENTA,
    YELLOW,
    WHITE
}

pub fn rgb_of_color(color:Color) -> [u8;3]{
    match color {
        Color::BLACK=> return [0, 0, 0],
        Color::BLUE=> return [0, 0, 255],
        Color::GREEN=> return [0, 255, 0],
        Color::CYAN=> return [0, 255, 255],
        Color::RED=> return [255, 0, 0],
        Color::MAGENTA=> return [255, 0, 255],
        Color::YELLOW=> return [255, 255, 0],
        Color::WHITE=> return [255, 255, 255]
    }
}

