mod utils;
use utils::config;
use utils::enums::colors::Color;

fn main() -> (){
    let jab_code : Vec<Vec<Color>> = encode("Test");
    if config::PRINT_CODE{
        jab_code.iter().for_each(|vec|{
            vec.iter().for_each(|&pixel|{
                print!("{}  ",pixel as u8);
            });
            println!();
        });
    }

}

//25x25 image support
//no options supported yet
fn encode(_message:&str) -> Vec<Vec<Color>>{
    let mut jab_code_primary : Vec<Vec<Color>> = initPallet(25,25);
    createFinderPattern(&mut jab_code_primary);

    return jab_code_primary;
}

/**Initializes Image-Pallet in give dimensions using Color.BLACK as default.
    Boundaries are inclusive.
**/
fn init_pallet(width:i32, height:i32) -> Vec<Vec<Color>>{
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
fn create_finder_pattern(pallet: &mut Vec<Vec<Color>>) -> (){

    let max_height = pallet.len();
    let max_width = pallet[1].len();

    //Finder-Pattern UL
    pallet[1][1] = Color::BLUE;
    pallet[1][2] = Color::BLUE;
    pallet[1][3] = Color::BLUE;

    pallet[2][1] = Color::BLUE;
    pallet[2][2] = Color::YELLOW;
    pallet[2][3] = Color::YELLOW;

    pallet[3][1] = Color::BLUE;
    pallet[3][2] = Color::YELLOW;

    pallet[3][3] = Color::BLUE;

    pallet[3][4] = Color::YELLOW;
    pallet[3][5] = Color::BLUE;

    pallet[4][3] = Color::YELLOW;
    pallet[4][4] = Color::YELLOW;
    pallet[4][5] = Color::BLUE;

    pallet[5][3] = Color::BLUE;
    pallet[5][4] = Color::BLUE;
    pallet[5][5] = Color::BLUE;

    //Finder-Pattern UR
    pallet[1][max_width - 6] = Color::GREEN;
    pallet[1][max_width - 5] = Color::GREEN;
    pallet[1][max_width - 4] = Color::GREEN;

    pallet[2][max_width - 6] = Color::GREEN;
    pallet[2][max_width - 5] = Color::MAGENTA;
    pallet[2][max_width - 4] = Color::MAGENTA;

    pallet[3][max_width - 6] = Color::GREEN;
    pallet[3][max_width - 5] = Color::MAGENTA;

    pallet[3][max_width - 4] = Color::GREEN;

    pallet[3][max_width - 3] = Color::MAGENTA;
    pallet[3][max_width - 2] = Color::GREEN;

    pallet[4][max_width - 4] = Color::MAGENTA;
    pallet[4][max_width - 3] = Color::MAGENTA;
    pallet[4][max_width - 2] = Color::GREEN;

    pallet[5][max_width - 4] = Color::GREEN;
    pallet[5][max_width - 3] = Color::GREEN;
    pallet[5][max_width - 2] = Color::GREEN;

    //Finder-Pattern LR
    pallet[max_height - 6][max_width - 4] = Color::MAGENTA;
    pallet[max_height - 6][max_width - 3] = Color::MAGENTA;
    pallet[max_height - 6][max_width - 2] = Color::MAGENTA;

    pallet[max_height - 5][max_width - 4] = Color::GREEN;
    pallet[max_height - 5][max_width - 3] = Color::GREEN;
    pallet[max_height - 5][max_width - 2] = Color::MAGENTA;

    pallet[max_height - 4][max_width - 6] = Color::MAGENTA;
    pallet[max_height - 4][max_width - 5] = Color::GREEN;

    pallet[max_height - 4][max_width - 4] = Color::MAGENTA;

    pallet[max_height - 4][max_width - 3] = Color::GREEN;
    pallet[max_height - 4][max_width - 2] = Color::MAGENTA;

    pallet[max_height - 3][max_width - 6] = Color::MAGENTA;
    pallet[max_height - 3][max_width - 5] = Color::GREEN;
    pallet[max_height - 3][max_width - 4] = Color::GREEN;

    pallet[max_height - 2][max_width - 6] = Color::MAGENTA;
    pallet[max_height - 2][max_width - 5] = Color::MAGENTA;
    pallet[max_height - 2][max_width - 4] = Color::MAGENTA;

    //Finder-Pattern LL
    pallet[max_height - 6][3] = Color::YELLOW;
    pallet[max_height - 6][4] = Color::YELLOW;
    pallet[max_height - 6][5] = Color::YELLOW;

    pallet[max_height - 5][3] = Color::BLUE;
    pallet[max_height - 5][4] = Color::BLUE;
    pallet[max_height - 5][5] = Color::YELLOW;

    pallet[max_height - 4][1] = Color::YELLOW;
    pallet[max_height - 4][2] = Color::BLUE;

    pallet[max_height - 4][3] = Color::YELLOW;

    pallet[max_height - 4][4] = Color::BLUE;
    pallet[max_height - 4][5] = Color::YELLOW;

    pallet[max_height - 3][1] = Color::YELLOW;
    pallet[max_height - 3][2] = Color::BLUE;
    pallet[max_height - 3][3] = Color::BLUE;

    pallet[max_height - 2][1] = Color::YELLOW;
    pallet[max_height - 2][2] = Color::YELLOW;
    pallet[max_height - 2][3] = Color::YELLOW;
}
