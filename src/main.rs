// use fltk::{app, window, frame, prelude::*, image::RgbImage, enums::{ColorDepth, Color}};
use macroquad::prelude::{load_ttf_font, screen_width, screen_height, Texture2D, draw_texture_ex, DrawTextureParams, draw_text_ex, TextParams, Color, get_fps, next_frame};
#[path = "render.rs"] mod render;
use render::*;


fn basic_scene() -> Scene {
    let mut scene = Scene::new();
    scene.set_camera(Camera::new(
        Point3::new(0., 2., 0.),
        Vec3::new(-0.15, 1.8, 1.0),
    ));

    scene.add(Primitive::cuboid(Point3::new(0.0, 1., 6.), Vec3::new(0.5, 0.75, 0.5), Rgb::new(1.0, 0.2, 0.2)));
    scene.add(Primitive::sphere(Point3::new(-1.25, 1., 6.), 0.8, Rgb::new(0.2, 1.0, 0.2)));
    scene.add(Primitive::aa_plane(Axis::Y, 0., Rgb::new(1.0, 1.0, 1.0)));
    scene.add_light(Point3::new(6., 5., -6.));
    return scene
}



#[macroquad::main("Rusty Raymarching")]
async fn main() {
    let text_font = load_ttf_font("assets/Monaco.ttf").await.unwrap();
    let scene = basic_scene();
    loop {
        let image = render_frame(&scene, (screen_width() / 2.) as i32, (screen_height() / 2.) as i32);
        let texture = Texture2D::from_image(&image);
        
        draw_texture_ex(
            texture,
            0.0, 0.0, Color::new(1.0, 1.0, 1.0, 1.0),
            DrawTextureParams{dest_size: Some(macroquad::math::Vec2::new(screen_width(), screen_height())), ..Default::default()}
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
