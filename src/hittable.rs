use crate::{ray::Ray, vec3::{Point3, Vec3}};



#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}