use ray_tracer::*;

const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

#[test]
fn creating_a_canvas() {
    let c = canvas(10, 20);

    assert_eq!(c.width(), 10);
    assert_eq!(c.height(), 20);

    for col in 0..c.width() {
        for row in 0..c.height() {
            assert_eq!(c.get(col, row), BLACK);
        }
    }
}

#[test]
fn writing_pixels_to_a_canvas() {
    let mut c = canvas(10, 20);
    let red = color(1, 0, 0);

    c.set(2, 3, red);

    assert_eq!(c.get(2, 3), red);
}
