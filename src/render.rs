#[path = "scene.rs"] mod scene;
pub use scene::*;
use macroquad::prelude::{Image, Color};
// use image::{ImageBuffer, self};


const MAX_STEPS: usize = 100;
const MAX_DIST: f32 = 100.;
const SURF_DIST: f32 = 0.01;


pub fn march(scene: &Scene, origin: Vec3, direction: Vec3) -> f32 {
    let mut dist = 0.;

    for i in 0..MAX_STEPS {
        let p = origin + direction * dist;
        let ds = scene.signed_distance(p);
        dist += ds;
        if (dist > MAX_DIST) || (ds < SURF_DIST) {
            break;
        }
    }

    return dist;
}

pub fn get_normal(scene: &Scene, p: Point3) -> Vec3 {
    let d = scene.signed_distance(p);
    let e = Vec2::new(0.01, 0.0);
    let n = Vec3::new(
        d - scene.signed_distance(p - e.xyy()),
        d - scene.signed_distance(p - e.yxy()),
        d - scene.signed_distance(p - e.yyx())
    );
    return n.normalize();
}

pub fn get_light(scene: &Scene, p: Point3) -> f32 {
    let light_pos = scene.lights[0];
    let l = (light_pos - p).normalize();
    let n = get_normal(scene, p);

    let mut diffuse = clamp(n.dot(l), 0., 1.);
    let d = march(scene, p + n * SURF_DIST * 2., l);
    if d < (light_pos - p).length() {
        diffuse *= 0.1;
    }
    return diffuse;
}


pub fn write_color(imgbuf: &mut Image, x: i32, y: i32, color: Rgb) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let ir = clamp(r, 0.0, 0.999);
    let ig = clamp(g, 0.0, 0.999);
    let ib = clamp(b, 0.0, 0.999);

    imgbuf.set_pixel(
        x as u32,
        y as u32,
        Color::new(ir, ig, ib, 1.0)
    );
}


pub fn render_frame(image_width: i32, image_height: i32) -> Image {
    let resolution = Vec2::new(image_width as f32, image_height as f32);
    let mut imgbuf = Image::gen_image_color(image_width as u16, image_height as u16, Color::new(0.0, 0.0, 0.0, 1.0));

    let mut scene = Scene::new();
    scene.add(Primitive::sphere(Point3::new(0., 1., 6.), 1.));
    scene.add(Primitive::aa_plane(Axis::Y, 0.));
    scene.add_light(Point3::new(0., 5., 6.));

    for x in 0..image_width {
        for y in 0..image_height {
            let uv = (Vec2::new(x as f32, (image_height - y) as f32) - resolution * 0.5) / resolution.y;

            let origin = Vec3::new(0., 1., 0.);
            let direction = Vec3::new(uv.x, uv.y, 1.).normalize();

            let d = march(&scene, origin, direction);
            let p = origin + direction * d;
            let diffuse = get_light(&scene, p);

            let color = Vec3::new(diffuse, diffuse, diffuse);
            write_color(&mut imgbuf, x, y, color);
        };
    };
    return imgbuf;
}