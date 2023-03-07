#[path = "camera.rs"] mod camera;
pub use camera::*;



pub struct Scene {
    pub objects: Vec<Primitive>,
    pub lights: Vec<Vec3>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            camera: Camera::new(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.)),
        }
    }
    pub fn add(&mut self, object: Primitive) {
        self.objects.push(object);
    }
    pub fn add_light(&mut self, point: Point3) {
        self.lights.push(point);
    }

    pub fn min(&self, a: (f32, Rgb), b: (f32, Rgb)) -> (f32, Rgb) {
        if a.0 < b.0 { a } else { b }
    }
    pub fn max(&self, a: (f32, Rgb), b: (f32, Rgb)) -> (f32, Rgb) {
        if a.0 > b.0 { a } else { b }
    }
    pub fn smooth_max(&self, a: (f32, Rgb), b: (f32, Rgb), k: f32) -> (f32, Rgb) {
        let t = ((k * a.0).exp() + (k * b.0).exp()).log10() / k;
        (t, a.1.lerp(b.1, (t - a.0) / (-b.0 - a.0)))
    }
    pub fn smooth_min(&self, a: (f32, Rgb), b: (f32, Rgb), k: f32) -> (f32, Rgb) {
        let t = -(((k * -a.0).exp() + (k * -b.0).exp()).log10() / k);
        (t, a.1.lerp(b.1, (t - a.0) / (-b.0 - a.0)))
    }
    pub fn signed_distance(&self, p: Point3) -> (f32, Rgb) {
        // let dist = self.objects.iter()
        //     .map(|x| (x, x.signed_distance(p)))
        //     .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        //     .unwrap();
        // let mut dist = (f32::MAX, Rgb::origin());
        // for obj in self.objects.iter() {
        //     let d = obj.signed_distance(p);
        //     dist = self.min(dist, (d, obj.material));
        // }
        let mut dist = self.smooth_min((self.objects[0].signed_distance(p), self.objects[0].material), (self.objects[1].signed_distance(p), self.objects[1].material), 3.0);
        dist = self.min(dist, (self.objects[2].signed_distance(p), self.objects[2].material));
        return dist;
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
}