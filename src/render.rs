#[path = "primitive.rs"] mod primitive;
use image::{ImageBuffer, self};
pub use primitive::*;


pub fn write_color(imgbuf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, x: i32, y: i32, color: Rgb) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let ir = 256.0 * clamp(r, 0.0, 0.999);
    let ig = 256.0 * clamp(g, 0.0, 0.999);
    let ib = 256.0 * clamp(b, 0.0, 0.999);

    imgbuf.put_pixel(
        x as u32,
        y as u32,
        image::Rgb([ir as u8, ig as u8, ib as u8])
    );
}





pub fn render_frame(image_width: i32, image_height: i32) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let resolution = Vec2::new(image_width as f32, image_height as f32);
    let mut imgbuf = ImageBuffer::new(image_width as u32, image_height as u32);
    for x in 0..image_width {
        for y in 0..image_height {
            let uv = (Vec2::new(x as f32, y as f32) - resolution * 0.5) / resolution.y;

            let color = Vec3::origin();

            let ro = Vec3::new(0., 1., 0.);
            let rd = Vec3::new(uv.x, uv.y, 1.).normalize();

            write_color(&mut imgbuf, x, y, color);
        };
    };
    return imgbuf;
}