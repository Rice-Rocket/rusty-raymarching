#[path = "scene.rs"] mod scene;
pub use scene::*;
use macroquad::prelude::{Image, Color};
use rayon::prelude::*;
use std::sync::Mutex;
// use image::{ImageBuffer, self};


const MAX_STEPS: usize = 64;
const MAX_DIST: f32 = 100.;
const SURF_DIST: f32 = 0.01;


pub fn march(scene: &Scene, origin: Vec3, direction: Vec3) -> f32 {
    let mut dist = 0.;

    for _ in 0..MAX_STEPS {
        let p = origin + direction * dist;
        let ds = scene.signed_distance(p).0;
        dist += ds;
        if ds < SURF_DIST {
            break;
        }
        if dist > MAX_DIST {
            break;
        }
    }

    return dist;
}

pub fn get_normal(scene: &Scene, p: Point3) -> (Rgb, Vec3) {
    let (d, color) = scene.signed_distance(p);
    let e = Vec2::new(0.001, 0.0);
    let n = Vec3::new(
        d - scene.signed_distance(p - e.xyy()).0,
        d - scene.signed_distance(p - e.yxy()).0,
        d - scene.signed_distance(p - e.yyx()).0
    );
    return (n.normalize(), color);
}

pub fn get_light(scene: &Scene, p: Point3) -> Rgb {
    let mut total_diffuse = Rgb::origin();
    let k = 16.0;
    let darkest = 0.05;
    for light_pos in scene.lights.iter() {
        let l = (*light_pos - p).normalize();
        let (n, hit_color) = get_normal(scene, p);
        let mut diffuse = hit_color * clamp(n.dot(l), darkest, 1.);

        let origin = p + n * SURF_DIST;
        let mut dist = scene.signed_distance(origin + l).0;
        let mut res = 1.0f32;
        let mut ph = 1e+20f32;
        for _ in 0..MAX_STEPS {
            let p = origin + l * dist;
            let ds = scene.signed_distance(p).0;
            dist += ds;
            if ds < SURF_DIST {
                res = darkest;
                break;
            }
            let y = ds * ds / (2.0 * ph);
            let d = (ds * ds - y * y).sqrt();
            res = res.min(k * d / (dist - y).max(0.0)).max(darkest);
            ph = ds;
            if dist > MAX_DIST {
                break;
            }
        };
        diffuse = diffuse * res;
        total_diffuse = total_diffuse + diffuse / scene.lights.len() as f32;
    }
    return total_diffuse;
}


pub fn write_color(imgbuf: &mut Image, x: i32, y: i32, color: Rgb) {
    let r = color.x.sqrt();
    let g = color.y.sqrt();
    let b = color.z.sqrt();

    let ir = clamp(r, 0.0, 0.999);
    let ig = clamp(g, 0.0, 0.999);
    let ib = clamp(b, 0.0, 0.999);

    imgbuf.set_pixel(
        x as u32,
        y as u32,
        Color::new(ir, ig, ib, 1.0)
    );
}


pub fn render_frame(scene: &Scene, image_width: i32, image_height: i32) -> Image {
    let resolution = Vec2::new(image_width as f32, image_height as f32);
    let imgbuf = Mutex::new(Image::gen_image_color(image_width as u16, image_height as u16, Color::new(0.0, 0.0, 0.0, 1.0)));

    (0..image_width).into_par_iter().for_each(|x| {
        for y in 0..image_height {
            // let uv = (Vec2::new(x as f32, (image_height - y) as f32) - resolution * 0.5) / resolution.y;
            let uv = Vec2::new(x as f32 * (image_width as f32 / image_height as f32), (image_height - y) as f32) / resolution;

            let (origin, direction) = scene.camera.get_ray(uv);

            let d = march(&scene, origin, direction);
            let p = origin + direction * d;
            let diffuse = get_light(&scene, p);

            let color = diffuse;
            write_color(&mut imgbuf.lock().unwrap(), x, y, color);
        };
    });
    return imgbuf.lock().unwrap().clone();
}