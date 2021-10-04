mod utils;
use utils::config;
use utils::enums::colors::Color;
use utils::image_maker;
use utils::structs::code;


fn main() -> (){
    let jab_code: code::Code = code::Code {
        height: config::CODE_HEIGHT,
        width: config::CODE_WIDTH,
        data: encode("Test")
    };

    if config::PRINT_CODE{
        jab_code.data.iter().for_each(|vec|{
            vec.iter().for_each(|&pixel|{
                print!("{}  ",pixel as u8);
            });
            println!();
        });
    }
    if config::RENDER_TO_PNG {
        image_maker::render_to_png(jab_code.data);
    }
}

//25x25 image support
//no options supported yet
fn encode(_message:&str) -> Vec<Vec<Color>>{
    let mut jab_code_primary : Vec<Vec<Color>> = init_pallet(config::CODE_WIDTH,config::CODE_HEIGHT);

    let maxLength: u16 = (jab_code_primary.len()-4) as u16;

    create_finder_pattern(3, 3, Color::BLUE, Color::YELLOW, false, &mut jab_code_primary);
    create_finder_pattern(maxLength, 3, Color::GREEN, Color::MAGENTA, false, &mut jab_code_primary);
    create_finder_pattern(3, maxLength, Color::YELLOW, Color::BLUE, true, &mut jab_code_primary);
    create_finder_pattern(maxLength, maxLength, Color::MAGENTA, Color::GREEN, true, &mut jab_code_primary);


    return jab_code_primary;
}

/**Initializes Image-Pallet in give dimensions using Color.BLACK as default.
    Boundaries are inclusive.
**/
fn init_pallet(width:u32, height:u32) -> Vec<Vec<Color>>{
    let mut pallet : Vec<Vec<Color>> = Vec::new();
    for _ in 1..=height {
        let mut row : Vec<Color> = Vec::new();
        for _ in 1..=width {
            row.push(Color::BLACK);
        }
        pallet.push(row);
    }

    return pallet;
}

//Code Finder-Pattern
fn create_finder_pattern(center_x: u16, center_y: u16, primary: Color, secondary: Color, invert: bool, pallet: &mut Vec<Vec<Color>>) -> (){

    //Define the finder pattern trough a 2D vector
    let mut pattern = vec![
        vec![1, 1, 1, 255, 255],
        vec![1, 2, 2, 255, 255],
        vec![1, 2, 1, 2, 1],
        vec![255, 255, 2, 2, 1],
        vec![255, 255, 1, 1, 1],
    ];

    //invert pattern if we need the lower patterns
    if invert{
        pattern.reverse();
    }

    //Define offset of patterns
    let mut x: usize = (center_x - 2).into();
    let mut y: usize = (center_y - 2).into();

    for i in 0..=pattern.len()-1 {
        x = (center_x - 2).into();
        for j in 0..=pattern[i].len()-1 {
            //Depent coloring of module trough a number
            if pattern[i][j] == 1{
                pallet[x][y] = primary;
            }else if pattern[i][j] == 2 {
                pallet[x][y] = secondary;
            }

            x = x + 1;
        }
        y = y + 1;
    }


}


