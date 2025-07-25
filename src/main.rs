mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;

use std::{io, mem::discriminant};
use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = vec3::dot(ray.direction(), oc);
    let c = oc.length_squared() - (radius * radius);
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(ray : &Ray) -> Color {

    let t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, ray);
    if t > 0.0 {
        let n = vec3::unit_vector(ray.at(t) - Vec3::new(0.0,0.0,-1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    }

    let unit_direction = vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {

    //Define Image width and Height
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32 ;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_point = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0,0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0,0.0, focal_point);

    //Print ppm headers
    print!("P3\n{} {}\n255\n",IMAGE_WIDTH, IMAGE_HEIGHT);

    //Printe Shader Hello world
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin
            );

            let pixel_color = ray_color(&r);
            color::write_color(&mut std::io::stdout(), pixel_color);
        }
    }
    eprint!("\nDone!\n");
}