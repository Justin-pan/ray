use crate::vec3f;
use crate::ray;

use rand;

fn random_in_unit_disk() -> vec3f::Vec3f32
{
    let mut p = vec3f::Vec3f32::new_from_points(1.5, 1.5, 1.5);
    while p.dot_product(&p) >= 1.0
    {
        p = (vec3f::Vec3f32::new_from_points(rand::random::<f32>(),
                                             rand::random::<f32>(),
                                             rand::random::<f32>()) * 2.0) -
            vec3f::Vec3f32::new_from_points(1.0, 1.0, 0.0);

    }
    p
}

pub struct Camera
{
    origin: vec3f::Vec3f32,
    lower_left_corner: vec3f::Vec3f32,
    horizontal: vec3f::Vec3f32,
    vertical: vec3f::Vec3f32,
    u: vec3f::Vec3f32,
    v: vec3f::Vec3f32,
    w: vec3f::Vec3f32,
    lens_radius: f32,
}

impl Camera
{
    pub fn new(look_from: vec3f::Vec3f32, look_at: vec3f::Vec3f32,
               vup: vec3f::Vec3f32, vfov: f32, aspect: f32,
               aperature: f32, focus_dist: f32) -> Camera
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
            lower_left_corner: look_from - u * (focus_dist * half_width)
                - v * (focus_dist * half_height) - w * focus_dist,
            horizontal: u * (focus_dist * (2.0 * half_width)),
            vertical: v * (focus_dist * (2.0 * half_height)),
            u: u,
            v: v,
            w: w,
            lens_radius: aperature / 2.0
        }
    }

    fn direction_from_camera(&self, u: f32, v: f32) -> vec3f::Vec3f32
    {
        self.lower_left_corner + self.horizontal * u +
            self.vertical * v - self.origin
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray
    {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        ray::Ray::new_from_vector(&(self.origin + offset), &(self.direction_from_camera(u, v) - offset))
    }
}
