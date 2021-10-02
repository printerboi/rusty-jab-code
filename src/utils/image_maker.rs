extern crate image;
use std::fs;
use crate::utils::enums::colors::*;

pub fn create_image_buffer(imgx: u32, imgy: u32) -> image::RgbImage{
    return image::ImageBuffer::new(imgx, imgy);
}

pub fn save_image_buffer(buffer: image::RgbImage) -> bool {
    attempt_to_create_folder();
    let attempt = buffer.save("render/code.png");

    //Error-handling needs to be refactored later
    match attempt {
        Ok(()) => return true,
        Err(error) => return false,
    };
}

fn attempt_to_create_folder() -> bool{
    let attempt = fs::create_dir("render");

    //Error-handling needs to be refactored later
    match attempt {
        Ok(()) => return true,
        Err(error) => return false,
    };
}

pub fn color_pixel(buffer: &mut image::RgbImage, x: u32, y: u32, color: u8){
    buffer.put_pixel(x, y, image::Rgb([255, 255, 255]));
}