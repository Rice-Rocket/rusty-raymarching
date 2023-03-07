#[path = "primitive.rs"] mod primitive;
pub use primitive::*;



pub struct Scene {
    pub objects: Vec<Primitive>,
    pub lights: Vec<Vec3>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Primitive) {
        self.objects.push(object);
    }
    pub fn add_light(&mut self, point: Point3) {
        self.lights.push(point);
    }
    pub fn signed_distance(&self, p: Point3) -> f32 {
        let dist = self.objects[0].signed_distance(p).min(self.objects[1].signed_distance(p));
        return dist;
    }
}