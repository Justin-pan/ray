use crate::vec3f;
use crate::ray;

pub struct Camera
{
    origin: vec3f::Vec3f32,
    lower_left_corner: vec3f::Vec3f32,
    horizontal: vec3f::Vec3f32,
    vertical: vec3f::Vec3f32,
}

impl Camera
{
    pub fn new(look_from: vec3f::Vec3f32, look_at: vec3f::Vec3f32,
               vup: vec3f::Vec3f32, vfov: f32, aspect: f32) -> Camera
    {
        let u: vec3f::Vec3f32;
        let v: vec3f::Vec3f32;
        let w: vec3f::Vec3f32;
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        w = (look_from - look_at).unit_vector();
        u = (vup.cross_product(&w)).unit_vector();
        v = w.cross_product(&u);
        Camera
        {
            origin: look_from,
            lower_left_corner: look_from - u * half_width - v * half_height - w,
            horizontal: u * (2.0 * half_width),
            vertical: v * (2.0 * half_height)
        }
    }

    fn direction_from_camera(&self, u: f32, v: f32) -> vec3f::Vec3f32
    {
        self.lower_left_corner + self.horizontal * u +
            self.vertical * v - self.origin
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray
    {
        ray::Ray::new_from_vector(&self.origin, &self.direction_from_camera(u, v))
    }
}
