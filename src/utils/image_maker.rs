extern crate image;
use crate::utils::enums::colors::*;

fn create_image_buffer(imgx: u32, imgy: u32) -> image::RgbImage{
    return image::ImageBuffer::new(imgx, imgy);
}

fn save_image_buffer(buffer: image::RgbImage) {
    buffer.save("render/code.png");
}

fn color_pixel(buffer: &mut image::RgbImage, x: u32, y: u32, color: Color){
    let mut pixel= image::Rgb(rgb_of_color(color));
    buffer.put_pixel(x, y, pixel);
}

pub fn render_to_png(image:Vec<Vec<Color>>) -> (){
    let mut buffer = create_image_buffer(image.len() as u32,image[0].len() as u32);
    for i in 0..image.len(){
        for j in 0..image[i].len(){
            color_pixel(&mut buffer,i as u32,j as u32,image[i][j]);
        }
    }

    save_image_buffer(buffer);
}