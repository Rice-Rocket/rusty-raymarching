#[path = "vec3.rs"] mod vec3;
pub use vec3::*;


pub enum PrimitiveType {
    Sphere(Point3, f32),
    AAPlane(Axis, f32),
    Cuboid(Vec3, Vec3),
}



pub struct Primitive {
    pub ptype: PrimitiveType,
    pub material: Rgb,
}

impl Primitive {
    pub fn sphere(center: Point3, radius: f32, material: Rgb) -> Self {
        Self {
            ptype: PrimitiveType::Sphere(center, radius),
            material
        }
    }
    pub fn sphere_sd(&self, p: Point3, center: Point3, radius: f32) -> f32 {
        (p - center).length() - radius
    }

    pub fn aa_plane(axis: Axis, k: f32, material: Rgb) -> Self {
        Self {
            ptype: PrimitiveType::AAPlane(axis, k),
            material
        }
    }
    pub fn aa_plane_sd(&self, p: Point3, axis: Axis, k: f32) -> f32 {
        p[axis] + k
    }

    pub fn cuboid(center: Point3, dims: Vec3, material: Rgb) -> Self {
        Self {
            ptype: PrimitiveType::Cuboid(center, dims),
            material
        }
    }
    pub fn cuboid_sd(&self, p: Point3, center: Point3, dims: Vec3) -> f32 {
        return ((p - center).abs() - dims).max(0.0).length()
    }

    pub fn signed_distance(&self, p: Point3) -> f32 {
        match &self.ptype {
            PrimitiveType::Sphere(center, radius) => self.sphere_sd(p, *center, *radius),
            PrimitiveType::AAPlane(axis, k) => self.aa_plane_sd(p, *axis, *k),
            PrimitiveType::Cuboid(center, dims) => self.cuboid_sd(p, *center, *dims),
        }
    }
}