extern crate image;

pub fn create_image_buffer(imgx: u32, imgy: u32) -> image::RgbImage{
    return image::ImageBuffer::new(imgx, imgy);
}

pub fn save_image_buffer(buffer: image::RgbImage) {
    buffer.save("render/code.png");
}

pub fn color_pixel(buffer: &mut image::RgbImage, x: u32, y: u32, color: u8){
    buffer.put_pixel(x, y, image::Rgb([255, 255, 255]));
}