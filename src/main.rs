use fltk::{app, window, frame, prelude::*, image::RgbImage, enums::{ColorDepth, Color}};
#[path = "render.rs"] mod render;
use render::*;



fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Oxy);
    let mut window = window::Window::new(100, 100, 1280, 720, "Rusty Raymarching");

    let image = render_frame(window.width(), window.height());
    let rgbimage = RgbImage::new(&image.into_raw(), window.width(), window.height(), ColorDepth::Rgb8).unwrap();

    let mut image_frame = frame::Frame::new(0, 0, window.width(), window.height(), "");
    image_frame.set_image(Some(rgbimage));

    window.make_resizable(true);
    window.end();
    window.show();

    window.set_color(Color::Black);

    app.run().unwrap();
}
