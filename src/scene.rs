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
    pub fn signed_distance(&self, p: Point3) -> (&Primitive, f32) {
        // let dist = self.objects[0].signed_distance(p).min(self.objects[1].signed_distance(p));
        let dist = self.objects.iter()
            .map(|x| (x, x.signed_distance(p)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();
        return dist;
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
}