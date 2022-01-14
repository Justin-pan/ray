#![allow(dead_code)]
mod vec3f;
mod mat;
mod ray;
mod camera;

use std::fs;
use std::io::Write;
use rand;

enum Material
{
    Lambertian,
    Metal,
    Dielectric,
}

struct HitRecord
{
    t: f32,
    p: vec3f::Vec3f32,
    normal: vec3f::Vec3f32,
}

struct Sphere
{
    centre: vec3f::Vec3f32,
    radius: f32,
    material: Material,
    albedo: vec3f::Vec3f32,
    fuzz: f32,
    refraction: f32
}

fn random_in_unit_sphere() -> vec3f::Vec3f32
{
    let mut p = vec3f::Vec3f32::new_from_points(1.5, 1.5, 1.5);
    while p.squared_length() >= 1f32
    {
        p = (vec3f::Vec3f32::new_from_points(rand::random::<f32>(),
                                             rand::random::<f32>(),
                                             rand::random::<f32>()) * 2.0) -
            vec3f::Vec3f32::new_from_points(1.0, 1.0, 1.0);
    }
    p
}

fn dot (first: &vec3f::Vec3f32, second: &vec3f::Vec3f32) -> f32
{
    (first.x * second.x) + (first.y * second.y) + (first.z * second.z)
}

fn hit_sphere(center: &vec3f::Vec3f32, radius: f32, r: &mut ray::Ray,
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
            rec.normal = (rec.p - *center) / radius;
            return true;
        }
        temp = (-b + discriminant.sqrt()) / a;
        if temp < tmax && temp > tmin
        {
            rec.t = temp;
            rec.p = r.point_at_parameter(&rec.t);
            rec.normal = (rec.p - *center) / radius;
            return true;
        }
    }
    false
}

fn color(r: &mut ray::Ray, spheres: &[Sphere],
         tmin: f32, tmax: f32, depth: i32) -> vec3f::Vec3f32
{
    let mut rec = HitRecord
    {
        t: 0.0f32,
        p: vec3f::Vec3f32::zeroes(),
        normal: vec3f::Vec3f32::zeroes(),
    };
    let mut hit_anything = false;
    let mut closest_so_far = tmax;
    let mut current_sphere = &spheres[0];
    for i in spheres
    {
        if hit_sphere(&i.centre, i.radius, r, tmin, closest_so_far,
                      &mut rec)
        {
            hit_anything = true;
            closest_so_far = rec.t;
            current_sphere = &i;
        }
    }

    if hit_anything
    {
        let mut scattered: ray::Ray;
        let attenuation: vec3f::Vec3f32;
        let scatter = match current_sphere.material
        {
            Material::Lambertian =>
            {
                let target = rec.p + rec.normal
                    + random_in_unit_sphere();
                scattered = ray::Ray::new_from_vector(&rec.p, &(target - rec.p));
                attenuation = current_sphere.albedo;
                true
            },

            Material::Metal =>
            {
                let unit_direction = r.direction().unit_vector();
                let reflected = unit_direction -
                    (rec.normal * 2f32 *
                     dot(&unit_direction, &rec.normal));
                scattered = ray::Ray::new_from_vector(&rec.p, &(reflected + (random_in_unit_sphere() * current_sphere.fuzz)));
                attenuation = current_sphere.albedo;
                dot(&scattered.direction(), &rec.normal) > 0f32
            },

            Material::Dielectric =>
            {
                attenuation = vec3f::Vec3f32::new_from_points(1.0, 1.0, 1.0);
                let outward_normal: vec3f::Vec3f32;
                let unit_direction = r.direction().unit_vector();
                let reflected = unit_direction -
                    (rec.normal * 2f32 *
                     dot(&unit_direction, &rec.normal));
                let ni_over_nt: f32;
                let mut refracted = vec3f::Vec3f32::zeroes();
                let reflect_prob: f32;
                let cosine: f32;
                if dot(&r.direction(), &rec.normal) > 0.0
                {
                    outward_normal = -rec.normal;
                    ni_over_nt = current_sphere.refraction;
                    cosine = ni_over_nt * dot(&r.direction(),
                                              &rec.normal) /
                        r.direction().length();
                }
                else
                {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0 / current_sphere.refraction;
                    cosine = -dot(&r.direction(), &rec.normal) /
                        r.direction().length();
                }
                let dt = dot(&unit_direction, &outward_normal);
                let discriminant = 1.0 - ni_over_nt * ni_over_nt *
                    (1.0 - dt * dt);
                if discriminant > 0.0
                {
                    refracted = (unit_direction -
                                 outward_normal * dt) *
                        ni_over_nt - outward_normal *
                        discriminant.sqrt();
                    let mut r0 = (1.0 - current_sphere.refraction)
                        / (1.0 + current_sphere.refraction);
                    r0 = r0 * r0;
                    reflect_prob = r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
                }
                else
                {
                    reflect_prob = 1.0;
                }
                if rand::random::<f32>() < reflect_prob
                {
                    scattered = ray::Ray::new_from_vector(&rec.p, &reflected);
                }
                else
                {
                    scattered = ray::Ray::new_from_vector(&rec.p, &refracted);
                }
                true
            }
        };

        if depth < 50 && scatter
        {
            return attenuation * color(&mut scattered,
                                       spheres, tmin, tmax,
                                       depth + 1);
        }
        else
        {
            return vec3f::Vec3f32::zeroes();
        }
    }
    else
    {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        let v1 = (vec3f::Vec3f32::new_from_points(1.0, 1.0, 1.0))
            * (1.0 - t);
        let v2 = (vec3f::Vec3f32::new_from_points(0.5, 0.7, 1.0))
            * t;
        v1 + v2
    }
}

fn main() {
    fs::create_dir_all("../data").unwrap();
    //let mut file = fs::File::create("j:/rust/data/foo.ppm").unwrap();
    let mut file = fs::File::create("/home/justin/Documents/ray/data/foo.ppm").unwrap();

    let mut world: Vec<Sphere> = Vec::new();
    world.push(
        Sphere
        {
            centre: vec3f::Vec3f32::new_from_points(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Material::Lambertian,
            albedo: vec3f::Vec3f32::new_from_points(0.5, 0.5, 0.5),
            fuzz: 1.0,
            refraction: 1.0
        }
    );

    for a in -11 .. 11
    {
        for b in -11 .. 11
        {
            let a_f = a as f32;
            let b_f = b as f32;
            let choose_mat = rand::random::<f32>();
            let centre = vec3f::Vec3f32::new_from_points(a_f + 0.9 *
                                                         rand::random::<f32>(),
                                                         0.2,
                                                         b_f + 0.9 *
                                                         rand::random::<f32>());
            let big_sphere_centre = vec3f::Vec3f32::new_from_points(4.0, 0.2, 0.0);
            if (centre - big_sphere_centre).length() > 0.9
            {
                if choose_mat < 0.8
                {
                    // diffuse
                    world.push(
                        Sphere
                        {
                            centre: centre,
                            radius: 0.2,
                            material: Material::Lambertian,
                            albedo: vec3f::Vec3f32::new_from_points(rand::random::<f32>() *
                                                                    rand::random::<f32>(),
                                                                    rand::random::<f32>() *
                                                                    rand::random::<f32>(),
                                                                    rand::random::<f32>() *
                                                                    rand::random::<f32>()),
                            fuzz: 1.0,
                            refraction: 1.0
                        }
                    );
                }
                else if choose_mat < 0.95
                {
                    // metal
                    world.push(
                        Sphere
                        {
                            centre: centre,
                            radius: 0.2,
                            material: Material::Metal,
                            albedo: vec3f::Vec3f32::new_from_points(0.5 * (1.0 + rand::random::<f32>()),
                                                                    0.5 * (1.0 + rand::random::<f32>()),
                                                                    0.5 * (1.0 + rand::random::<f32>())),
                            fuzz: 0.5 * rand::random::<f32>(),
                            refraction: 1.0
                        }
                    );
                }
                else
                {
                    // glass
                    world.push(
                        Sphere
                        {
                            centre: centre,
                            radius: 0.2,
                            material: Material::Dielectric,
                            albedo: vec3f::Vec3f32::new_from_points(1.0, 1.0, 1.0),
                            fuzz: 1.0,
                            refraction: 1.0 + rand::random::<f32>(),
                        }
                    );
                }
            }
        }
    }

    world.push(
        Sphere
        {
            centre: vec3f::Vec3f32::new_from_points(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Dielectric,
            albedo: vec3f::Vec3f32::new_from_points(1.0, 1.0, 1.0),
            fuzz: 1.0,
            refraction: 1.5
        }
    );
    world.push(
        Sphere
        {
            centre: vec3f::Vec3f32::new_from_points(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Lambertian,
            albedo: vec3f::Vec3f32::new_from_points(0.4, 0.2, 0.1),
            fuzz: 1.0,
            refraction: 1.0
        }
    );
    world.push(
        Sphere
        {
            centre: vec3f::Vec3f32::new_from_points(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Metal,
            albedo: vec3f::Vec3f32::new_from_points(0.8, 0.6, 0.5),
            fuzz: 0.0,
            refraction: 1.0
        }
    );

    let nx: f32 = 600f32;
    let ny: f32 = 300f32;
    let ns: f32 = 100f32;
    write!(&mut file, "P3\n {} {}\n255\n", nx, ny).unwrap();
    let look_from = vec3f::Vec3f32::new_from_points(13.0, 2.0, 3.0);
    let look_at = vec3f::Vec3f32::new_from_points(0.0, 0.0, 0.0);
    let vup = vec3f::Vec3f32::new_from_points(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let camera = camera::Camera::new(look_from, look_at,
                                     vup, 20.0, nx/ny, aperature,
                                     dist_to_focus);
    for j in (0 .. ny as u32).rev()
    {
        for i in 0 .. nx as u32
        {
            let mut col = vec3f::Vec3f32::zeroes();
            for _s in 0 .. ns as u32
            {
                let u = (i as f32 + rand::random::<f32>()) / nx;
                let v = (j as f32 + rand::random::<f32>()) / ny;
                let mut r = camera.get_ray(u, v);

                col += color(&mut r, &world, 0.001, f32::MAX, 0);
            }


            col /= ns;
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();
            col *= 255.99f32;
            col.write_vec_as_int(&mut file);
        }
    }
}
