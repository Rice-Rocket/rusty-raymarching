#[path = "primitive.rs"] mod primitive;
pub use primitive::*;



#[derive(Clone)]
pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left: Vec3
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3) -> Self {
        let w = (look_from - look_at).normalize();
        let u = Vec3::new(0., 1., 0.).cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u;
        let vertical = v;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Self {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left: lower_left,
        }
    }
    pub fn get_ray(&self, uv: Vec2) -> (Point3, Vec3) {
        (
            self.origin, 
            (self.lower_left + self.horizontal * uv.x + self.vertical * uv.y - self.origin).normalize()
        )
    }
}