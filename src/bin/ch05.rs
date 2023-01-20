use ray_tracer::geometry::*;
use ray_tracer::raytracer::*;
use ray_tracer::shape::*;

fn main() {
    let ray_origin = Point::new(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 256;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let wall_top = wall_size / 2.0;
    let wall_left = -wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let sphere = Sphere::new();

    for y in 0..canvas_pixels {
        let world_y = wall_top - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = wall_left + pixel_size * (x as f64);
            let wall_point = Point::new(world_x, world_y, wall_z);
            let direction = (wall_point - ray_origin).norm();
            let ray = Ray::new(ray_origin, direction);

            if sphere.intersect(ray).hit().is_some() {
                canvas.set(x, y, color)
            }
        }
    }
    let fname = "images/ch05.png";
    match canvas.save(fname) {
        Ok(_) => println!("wrote image to file {}", fname),
        Err(e) => println!("error writing file \"{}\": {}", fname, e),
    }
}
