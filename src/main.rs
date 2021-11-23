#![allow(dead_code)]
mod vec3f;
mod mat;
mod ray;

use std::fs;
use std::io::Write;

fn dot (first: &vec3f::Vec3f32, second: &vec3f::Vec3f32) -> f32
{
    (first.x * second.x) + (first.y * second.y) + (first.z * second.z)
}

fn hit_sphere(center: &vec3f::Vec3f32, radius: &f32, r: &mut ray::Ray) -> f32
{
    let oc = r.origin() - *center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0
    {
        -1.0
    }
    else
    {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(r: &mut ray::Ray) -> vec3f::Vec3f32
{
    let t = hit_sphere(&vec3f::Vec3f32::new_from_points(0.0, 0.0, -1.0), &0.5, r);
    if t > 0.0
    {
        let n = (r.point_at_parameter(&t) - vec3f::Vec3f32::new_from_points(0.0, 0.0, -1.0)).normalize();
        return vec3f::Vec3f32::new_from_points(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_direction = r.direction().normalize();
    let t = 0.5f32 * (unit_direction.y + 1.0f32);
    let v1 = (vec3f::Vec3f32::new_from_points(1.0, 1.0, 1.0)) * (1.0 - t);
    let v2 = (vec3f::Vec3f32::new_from_points(0.5, 0.7, 1.0)) * t;
    v1 + v2
}

fn main() {
    fs::create_dir_all("../data").unwrap();
    let mut file = fs::File::create("../data/foo.ppm").unwrap();

    let nx: f32 = 200f32;
    let ny: f32 = 100f32;
    write!(&mut file, "P3\n {} {}\n255\n", nx, ny).unwrap();
    let lower_left_corner = vec3f::Vec3f32::new_from_points(-2.0, -1.0, -1.0);
    let horizontal = vec3f::Vec3f32::new_from_points(4.0, 0.0, 0.0);
    let vertical = vec3f::Vec3f32::new_from_points(0.0, 2.0, 0.0);
    let origin = vec3f::Vec3f32::new_from_points(0.0, 0.0, 0.0);
    for j in (0 .. ny as u32).rev()
    {
        for i in 0 .. nx as u32
        {
            let u = (i as f32) / nx;
            let v = (j as f32) / ny;
            let mut r = ray::Ray::new_from_vector(&origin, &(lower_left_corner + horizontal * u + vertical * v));

            let mut col = color(&mut r);
            col *= 255.99f32;
            col.write_vec_as_int(&mut file);
        }
    }
}
