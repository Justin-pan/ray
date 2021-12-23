#![allow(dead_code)]
mod vec3f;
mod mat;
mod ray;

use std::fs;
use std::io::Write;
use rand::Rng;

struct HitRecord
{
    t: f32,
    p: vec3f::Vec3f32,
    normal: vec3f::Vec3f32,
}

fn dot (first: &vec3f::Vec3f32, second: &vec3f::Vec3f32) -> f32
{
    (first.x * second.x) + (first.y * second.y) + (first.z * second.z)
}

fn hit_sphere(center: &vec3f::Vec3f32, radius: &f32, r: &mut ray::Ray,
              tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool
{
    let oc = r.origin() - *center;
    let a = dot(&r.direction(), &r.direction());
    let b = dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    // This is to check if the ray intersects with the sphere at all
    // in certain cases, becuase a ray can go through the sphere, it can hit twice
    // or it can hit once, which resembles a quadratic formula
    let discriminant = b * b - a * c;
    // If the discriminant of the quadratic formula is less than 0, the roots
    // are not real, which means that the sphere was not hit
    if discriminant > 0.0
    {
        let mut temp = (-b - discriminant.sqrt()) / a;
        if temp < tmax && temp > tmin
        {
            rec.t = temp;
            rec.p = r.point_at_parameter(&rec.t);
            rec.normal = (rec.p - *center) / *radius;
            return true;
        }
        temp = (-b + discriminant.sqrt()) / a;
        if temp < tmax && temp > tmin
        {
            rec.t = temp;
            rec.p = r.point_at_parameter(&rec.t);
            rec.normal = (rec.p - *center) / *radius;
            return true;
        }
    }
    false
}

fn color(r: &mut ray::Ray, spheres: &[(f32, f32, f32, f32)], tmin: f32, tmax: f32) -> vec3f::Vec3f32
{
    let mut rec = HitRecord
    {
        t: 0.0f32,
        p: vec3f::Vec3f32::zeroes(),
        normal: vec3f::Vec3f32::zeroes(),
    };
    let mut hit_anything = false;
    let mut closest_so_far = tmax;
    for i in spheres
    {
        let (x, y, z, radius) = i;
        if hit_sphere(&vec3f::Vec3f32::new_from_points(*x, *y, *z), radius, r, tmin, closest_so_far, &mut rec)
        {
            hit_anything = true;
            closest_so_far = rec.t;
        }
    }

    if hit_anything
    {
        vec3f::Vec3f32::new_from_points(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0) * 0.5
    }
    else
    {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        let v1 = (vec3f::Vec3f32::new_from_points(1.0, 1.0, 1.0)) * (1.0 - t);
        let v2 = (vec3f::Vec3f32::new_from_points(0.5, 0.7, 1.0)) * t;
        v1 + v2
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    fs::create_dir_all("../data").unwrap();
    //let mut file = fs::File::create("j:/rust/data/foo.ppm").unwrap();
    let mut file = fs::File::create("/home/justin/Documents/ray/data/foo.ppm").unwrap();

    let world = [
        (0.0, 0.0, -1.0, 0.5),
        (0.0, -100.5, -1.0, 100.0)
    ];

    let nx: f32 = 600f32;
    let ny: f32 = 300f32;
    let ns: f32 = 50f32;
    write!(&mut file, "P3\n {} {}\n255\n", nx, ny).unwrap();
    let lower_left_corner = vec3f::Vec3f32::new_from_points(-2.0, -1.0, -1.0);
    let horizontal = vec3f::Vec3f32::new_from_points(4.0, 0.0, 0.0);
    let vertical = vec3f::Vec3f32::new_from_points(0.0, 2.0, 0.0);
    let origin = vec3f::Vec3f32::new_from_points(0.0, 0.0, 0.0);
    for j in (0 .. ny as u32).rev()
    {
        for i in 0 .. nx as u32
        {
            let mut col = vec3f::Vec3f32::zeroes();
            for _s in 0 .. ns as u32
            {
                let u = (i as f32 + rng.gen::<f32>()) / nx;
                let v = (j as f32 + rng.gen::<f32>()) / ny;
                let mut r = ray::Ray::new_from_vector(&origin, &(lower_left_corner + horizontal * u + vertical * v - origin));

                col += color(&mut r, &world, 0.0, f32::MAX);
            }


            col /= ns;
            col *= 255.99f32;
            col.write_vec_as_int(&mut file);
        }
    }
}
