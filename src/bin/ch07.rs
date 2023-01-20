use std::f64::consts::PI;
use std::time::Instant;

use ray_tracer::geometry::*;
use ray_tracer::raytracer::*;
use ray_tracer::shape::*;

fn main() {
    let material = Material::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let floor = Sphere::new()
        .with_transform(Matrix::new().scale(10, 0.01, 10))
        .with_material(material);

    let left_wall = Sphere::new()
        .with_transform(
            Matrix::new()
                .scale(10, 0.01, 10)
                .rotate_x(PI / 2.0)
                .rotate_y(-PI / 4.0)
                .translate(0, 0, 5),
        )
        .with_material(material);

    let right_wall = Sphere::new()
        .with_transform(
            Matrix::new()
                .scale(10, 0.01, 10)
                .rotate_x(PI / 2.0)
                .rotate_y(PI / 4.0)
                .translate(0, 0, 5),
        )
        .with_material(material);

    let middle = Sphere::new()
        .with_transform(Matrix::new().translate(-0.5, 1, 0.5))
        .with_material(
            Material::new()
                .with_color(Color::new(0.1, 1, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let right = Sphere::new()
        .with_transform(Matrix::new().scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .with_material(
            Material::new()
                .with_color(Color::new(0.5, 1, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let left = Sphere::new()
        .with_transform(
            Matrix::new()
                .scale(0.33, 0.33, 0.33)
                .translate(-1.5, 0.33, -0.75),
        )
        .with_material(
            Material::new()
                .with_color(Color::new(1, 0.8, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        );

    let world = World::new()
        .with_lights(vec![PointLight::new(
            Point::new(-10, 10, -10),
            color::WHITE,
        )])
        .with_objects(vec![floor, right_wall, left_wall])
        .with_objects(vec![middle, right, left]);

    let camera = Camera::new(1024, 512, PI / 3.0).with_view_transform(
        Point::new(0, 1.5, -5),
        Point::new(0, 1, 0),
        Vector::new(0, 1, 0),
    );

    let time = Instant::now();
    let canvas = camera.render(&world);
    println!("render took {:.2?}", time.elapsed());

    let fname = "images/ch07.png";
    match canvas.save(fname) {
        Ok(_) => println!("wrote image to file {}", fname),
        Err(e) => println!("error writing file \"{}\": {}", fname, e),
    }
}
