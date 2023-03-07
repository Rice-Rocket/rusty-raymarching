// use fltk::{app, window, frame, prelude::*, image::RgbImage, enums::{ColorDepth, Color}};
use macroquad::prelude::*;
#[path = "render.rs"] mod render;
use render::*;


#[macroquad::main("Rusty Raymarching")]
async fn main() {
    let text_font = load_ttf_font("assets/Monaco.ttf").await.unwrap();
    loop {
        let image = render_frame(screen_width() as i32, screen_height() as i32);
        let texture = Texture2D::from_image(&image);
        
        draw_texture_ex(
            texture,
            0.0, 0.0, WHITE,
            DrawTextureParams{dest_size: Some(vec2(screen_width(), screen_height())), ..Default::default()}
        );

        draw_text_ex(
            &format!("FPS: {}", get_fps()),
            8.0,
            24.0,
            TextParams{font: text_font, font_size: 16u16, color: Color::new(1.0, 1.0, 1.0, 0.7), ..Default::default()}
        );
        next_frame().await
    }
}
