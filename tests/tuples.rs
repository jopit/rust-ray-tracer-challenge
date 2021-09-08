use ray_tracer::*;

#[test]
fn creating_a_point() {
    let a = point(4.3, -4.2, 3.1);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
}

#[test]
fn creating_a_vector() {
    let a = vector(4.3, -4.2, 3.1);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
}

#[test]
fn adding_two_vectors() {
    let v1 = vector(3, -2, 5);
    let v2 = vector(-2, 3, 1);

    assert_eq!(v1 + v2, vector(1.0, 1.0, 6.0));
}

#[test]
fn adding_vector_and_point() {
    let v = vector(3, -2, 5);
    let p = point(-2, 3, 1);

    assert_eq!(v + p, point(1.0, 1.0, 6.0));
}

#[test]
fn adding_point_and_vector() {
    let p = vector(3, -2, 5);
    let v = point(-2, 3, 1);

    assert_eq!(p + v, point(1.0, 1.0, 6.0));
}

#[test]
fn subtracting_two_points() {
    let p1 = point(3, 2, 1);
    let p2 = point(5, 6, 7);

    assert_eq!(p1 - p2, vector(-2, -4, -6));
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = point(3, 2, 1);
    let v = vector(5, 6, 7);

    assert_eq!(p - v, point(-2, -4, -6));
}

#[test]
fn subtracting_two_vectors() {
    let v1 = vector(3, 2, 1);
    let v2 = vector(5, 6, 7);

    assert_eq!(v1 - v2, vector(-2, -4, -6));
}

#[test]
fn subtracting_a_vector_from_the_zero_vector() {
    let zero = vector(0, 0, 0);
    let v = vector(1, -2, 3);

    assert_eq!(zero - v, vector(-1, 2, -3));
}

#[test]
fn negating_a_vector() {
    let a = Vector {
        x: 1.0,
        y: -2.0,
        z: 3.0,
    };

    assert_eq!(-a, vector(-1.0, 2.0, -3.0));
}

#[test]
fn multiplying_a_vector_by_a_scalar() {
    let a = vector(1, -2, 3);
    assert_eq!(a * 3.5, vector(3.5, -7, 10.5));
}

#[test]
fn multiplying_a_vector_by_a_fraction() {
    let a = vector(1, -2, 3);
    assert_eq!(a * 0.5, vector(0.5, -1, 1.5));
}

#[test]
fn dividing_a_vector_by_a_scalar() {
    let a = vector(1, -2, 3);
    assert_eq!(a / 2, vector(0.5, -1, 1.5));
}

#[test]
fn computing_vector_magnitudes() {
    assert_eq!(vector(1, 0, 0).magnitude(), 1.0);
    assert_eq!(vector(0, 1, 0).magnitude(), 1.0);
    assert_eq!(vector(0, 0, 1).magnitude(), 1.0);
    assert_eq!(vector(1, 2, 3).magnitude(), (14.0 as f64).sqrt());
    assert_eq!(vector(-1, -2, -3).magnitude(), (14.0 as f64).sqrt());
}

#[test]
fn normalizing_vector_4_0_0() {
    let v = vector(4, 0, 0);
    assert_eq!(v.normalize(), vector(1, 0, 0));
}

#[test]
fn normalizing_vector_1_2_3() {
    let v = vector(1, 2, 3);
    assert_eq!(v.normalize(), vector(0.26726, 0.53452, 0.80178));
}

#[test]
fn magnitude_of_normalized_vector() {
    let v = vector(1, 2, 3);
    let norm = v.normalize();
    assert_eq!(norm.magnitude(), 1.0);
}

#[test]
fn dot_product_of_two_vectors() {
    let a = vector(1, 2, 3);
    let b = vector(2, 3, 4);
    assert_eq!(a.dot(b), 20.0);
}

#[test]
fn cross_product_of_two_vectors() {
    let a = vector(1, 2, 3);
    let b = vector(2, 3, 4);
    assert_eq!(a.cross(b), vector(-1, 2, -1));
    assert_eq!(b.cross(a), vector(1, -2, 1));
}

#[test]
fn colors_are_red_green_blue_tuples() {
    let c = color(-0.5, 0.4, 1.7);
    assert_eq!(c.red, -0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7);
}

#[test]
fn adding_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
}

#[test]
fn substracting_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
}

#[test]
fn multiplying_a_color_by_a_scalar() {
    let c = color(0.2, 0.3, 0.4);
    assert_eq!(c * 2, color(0.4, 0.6, 0.8));
}

#[test]
fn multiplying_colors() {
    let c1 = color(1, 0.2, 0.4);
    let c2 = color(0.9, 1, 0.1);
    assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    assert_eq!(c1.red, 1.0);
}
