use std::f64::consts::PI;

use ray_tracer::*;

fn main() {
    let width = 1024.0;
    let height = width;

    let radius = width * (3.0 / 8.0);
    let center = point(width / 2.0, 0.0, height / 2.0);

    let white = color(1.0, 1.0, 1.0);
    let red = color(1.0, 0.0, 0.0);

    let angle = PI / 6.0;
    let twelve = point(0, 0, 1);
    let transform = scale(radius, 1.0, radius).translate(center.x, center.y, center.z);

    let mut canvas = canvas(width as usize, height as usize);
    for hour in 0..12 {
        let point = (transform * rotate_y((hour as f64) * angle)) * twelve;
        let color = if hour == 0 || hour == 3 { red } else { white };

        let x = point.x as usize;
        let y = (height - point.z) as usize;
        canvas.set(x, y, color);
        canvas.set(x - 1, y, color);
        canvas.set(x + 1, y, color);
        canvas.set(x, y - 1, color);
        canvas.set(x, y + 1, color);
    }

    let fname = "images/ch04.png";
    match canvas.save(fname) {
        Ok(_) => println!("wrote image to file {}", fname),
        Err(e) => println!("error writing file \"{}\": {}", fname, e),
    }
}
