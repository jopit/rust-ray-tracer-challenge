use ray_tracer::geometry::*;

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn main() {
    let mut p = Projectile {
        position: Point::new(0, 1, 0),
        velocity: Vector::new(1, 1, 0).norm(),
    };

    let e = Environment {
        gravity: Vector::new(0, -0.1, 0),
        wind: Vector::new(-0.01, 0, 0),
    };

    while p.position.y() >= 0.0 {
        p = tick(&e, &p);
        println!("x: {:12.8} y: {:12.8}", p.position.x(), p.position.y());
    }
}
