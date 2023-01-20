use ray_tracer::geometry::*;
use ray_tracer::raytracer::*;
use ray_tracer::shape::*;

fn main() {
    let ray_origin = Point::new(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 512;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let wall_top = wall_size / 2.0;
    let wall_left = -wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let material = Material::new().with_color(Color::new(1.0, 0.2, 1.0));
    let sphere = Sphere::new().with_material(material);

    let light_position = Point::new(-10, 10, -10);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = wall_top - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = wall_left + pixel_size * (x as f64);
            let wall_point = Point::new(world_x, world_y, wall_z);
            let direction = (wall_point - ray_origin).norm();
            let ray = Ray::new(ray_origin, direction);

            let xs = sphere.intersect(ray);
            if let Some(hit) = xs.hit() {
                let point = ray.position(hit.t());
                let normal = hit.object().normal_at(point);
                let eye = -ray.direction();
                let color = hit.object().material().lighting(light, point, eye, normal);
                canvas.set(x, y, color);
            }
        }
    }

    let fname = "images/ch06.png";
    match canvas.save(fname) {
        Ok(_) => println!("wrote image to file {}", fname),
        Err(e) => println!("error writing file \"{}\": {}", fname, e),
    }
}
