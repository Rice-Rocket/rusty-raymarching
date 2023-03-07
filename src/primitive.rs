#[path = "vec3.rs"] mod vec3;
pub use vec3::*;


pub enum PrimitiveType {
    Sphere(Point3, f32),
    AAPlane(Axis, f32),
}



pub struct Primitive {
    pub ptype: PrimitiveType,
}

impl Primitive {
    pub fn sphere(center: Point3, radius: f32) -> Self {
        Self {
            ptype: PrimitiveType::Sphere(center, radius),
        }
    }
    pub fn sphere_sd(&self, p: Point3, center: Point3, radius: f32) -> f32 {
        (p - center).length() - radius
    }

    pub fn aa_plane(axis: Axis, k: f32) -> Self {
        Self {
            ptype: PrimitiveType::AAPlane(axis, k),
        }
    }
    pub fn aa_plane_sd(&self, p: Point3, axis: Axis, k: f32) -> f32 {
        p[axis] + k
    }

    pub fn signed_distance(&self, p: Point3) -> f32 {
        match &self.ptype {
            PrimitiveType::Sphere(center, radius) => self.sphere_sd(p, *center, *radius),
            PrimitiveType::AAPlane(axis, k) => self.aa_plane_sd(p, *axis, *k),
        }
    }
}