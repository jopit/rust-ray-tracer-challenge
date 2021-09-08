use ray_tracer::*;

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
    let start = point(0, 1, 0);
    let velocity = vector(1, 1.8, 0).normalize() * 11.25;

    let mut p = Projectile {
        position: start,
        velocity: velocity,
    };
    let gravity = vector(0, -0.1, 0);
    let wind = vector(-0.01, 0, 0);

    let e = Environment { gravity, wind };

    let color = color(1.0, 0.0, 0.0);
    let mut canvas = canvas(900, 550);

    while p.position.y > 0.0 {
        let x = p.position.x.round() as usize;
        let y = canvas.height() - (p.position.y.round() as usize);
        if x < canvas.width() || y < canvas.height() {
            canvas.set(x, y, color);
            if x as i32 - 1 >= 0 {
                canvas.set(x - 1, y, color);
            }
            if x + 1 < canvas.width() {
                canvas.set(x + 1, y, color);
            }
            if y as i32 - 1 >= 0 {
                canvas.set(x, y - 1, color);
            }
            if y + 1 < canvas.height() {
                canvas.set(x, y + 1, color);
            }
        } else {
            println!(
                "position out of bounds: px{} py{} (x: {} y: {})",
                p.position.x, p.position.y, x, y
            )
        }
        p = tick(&e, &p);
    }

    let fname = "images/ch02.png";
    canvas.save(fname).unwrap();
    println!("wrote image to file {}", fname);
}
