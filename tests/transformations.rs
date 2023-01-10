use std::f64::consts::{PI, SQRT_2};

use ray_tracer::geometry::*;

#[test]
fn multiplying_by_a_translation_matrix() {
    let transform = Matrix::new().translate(5, -3, 2);
    let p = Point::new(-3, 4, 5);

    assert_eq!(transform * p, Point::new(2, 1, 7));
}

#[test]
fn multiplying_by_the_inverse_of_a_translation_matrix() {
    let transform = Matrix::new().translate(5, -3, 2);
    let p = Point::new(-3, 4, 5);

    assert_eq!(transform.inverse() * p, Point::new(-8, 7, 3));
}

#[test]
fn translation_does_not_affect_vectors() {
    let transform = Matrix::new().translate(5, -3, 2);
    let v = Vector::new(-3, 4, 5);

    assert_eq!(transform * v, Vector::new(-3, 4, 5));
}

#[test]
fn a_scaling_matrix_applied_to_a_point() {
    let transform = Matrix::new().scale(2, 3, 4);
    let p = Point::new(-4, 6, 8);

    assert_eq!(transform * p, Point::new(-8, 18, 32));
}

#[test]
fn a_scaling_matrix_applied_to_a_vector() {
    let transform = Matrix::new().scale(2, 3, 4);
    let v = Vector::new(-4, 6, 8);

    assert_eq!(transform * v, Vector::new(-8, 18, 32));
}

#[test]
fn multiplying_by_the_inverse_of_a_scaling_matrix() {
    let transform = Matrix::new().scale(2, 3, 4);
    let v = Vector::new(-4, 6, 8);

    assert_eq!(transform.inverse() * v, Vector::new(-2, 2, 2));
}

#[test]
fn reflection_is_scaling_by_a_negative_value() {
    let transform = Matrix::new().scale(-1, 1, 1);
    let p = Point::new(2, 3, 4);

    assert_eq!(transform * p, Point::new(-2, 3, 4));
}

#[test]
fn rotating_a_point_around_the_x_axis() {
    let p = Point::new(0, 1, 0);
    let half_quarter = Matrix::new().rotate_x(PI / 4.0);
    let full_quarter = Matrix::new().rotate_x(PI / 2.0);

    assert_eq!(half_quarter * p, Point::new(0, SQRT_2 / 2.0, SQRT_2 / 2.0));
    assert_eq!(full_quarter * p, Point::new(0, 0, 1));
}

#[test]
fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
    let p = Point::new(0, 1, 0);
    let half_quarter = Matrix::new().rotate_x(PI / 4.0);

    assert_eq!(
        half_quarter.inverse() * p,
        Point::new(0, SQRT_2 / 2.0, -SQRT_2 / 2.0)
    );
}

#[test]
fn rotating_a_point_around_the_y_axis() {
    let p = Point::new(0, 0, 1);
    let half_quarter = Matrix::new().rotate_y(PI / 4.0);
    let full_quarter = Matrix::new().rotate_y(PI / 2.0);

    assert_eq!(half_quarter * p, Point::new(SQRT_2 / 2.0, 0, SQRT_2 / 2.0));
    assert_eq!(full_quarter * p, Point::new(1, 0, 0));
}

#[test]
fn rotating_a_point_around_the_z_axis() {
    let p = Point::new(0, 1, 0);
    let half_quarter = Matrix::new().rotate_z(PI / 4.0);
    let full_quarter = Matrix::new().rotate_z(PI / 2.0);

    assert_eq!(half_quarter * p, Point::new(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0));
    assert_eq!(full_quarter * p, Point::new(-1, 0, 0));
}

#[test]
fn a_shearing_transformation_moves_x_in_proportion_to_y() {
    let transform = Matrix::new().shear(1, 0, 0, 0, 0, 0);
    let p = Point::new(2, 3, 4);

    assert_eq!(transform * p, Point::new(5, 3, 4));
}

#[test]
fn a_shearing_transformation_moves_x_in_proportion_to_z() {
    let transform = Matrix::new().shear(0, 1, 0, 0, 0, 0);
    let p = Point::new(2, 3, 4);

    assert_eq!(transform * p, Point::new(6, 3, 4));
}

#[test]
fn a_shearing_transformation_moves_y_in_proportion_to_x() {
    let transform = Matrix::new().shear(0, 0, 1, 0, 0, 0);
    let p = Point::new(2, 3, 4);

    assert_eq!(transform * p, Point::new(2, 5, 4));
}

#[test]
fn a_shearing_transformation_moves_y_in_proportion_to_z() {
    let transform = Matrix::new().shear(0, 0, 0, 1, 0, 0);
    let p = Point::new(2, 3, 4);

    assert_eq!(transform * p, Point::new(2, 7, 4));
}

#[test]
fn a_shearing_transformation_moves_z_in_proportion_to_x() {
    let transform = Matrix::new().shear(0, 0, 0, 0, 1, 0);
    let p = Point::new(2, 3, 4);

    assert_eq!(transform * p, Point::new(2, 3, 6));
}

#[test]
fn a_shearing_transformation_moves_z_in_proportion_to_y() {
    let transform = Matrix::new().shear(0, 0, 0, 0, 0, 1);
    let p = Point::new(2, 3, 4);

    assert_eq!(transform * p, Point::new(2, 3, 7));
}

#[test]
fn individual_transformations_are_applied_in_sequence() {
    let p = Point::new(1, 0, 1);
    let a = Matrix::new().rotate_x(PI / 2.0);
    let b = Matrix::new().scale(5, 5, 5);
    let c = Matrix::new().translate(10, 5, 7);

    // apply rotation first
    let p2 = a * p;
    assert_eq!(p2, Point::new(1, -1, 0));

    // then apply scaling
    let p3 = b * p2;
    assert_eq!(p3, Point::new(5, -5, 0));

    // then apply translation
    let p4 = c * p3;
    assert_eq!(p4, Point::new(15, 0, 7));
}

#[test]
fn chained_transformations_must_be_applied_in_reverse_order() {
    let p = Point::new(1, 0, 1);
    let a = Matrix::new().rotate_x(PI / 2.0);
    let b = Matrix::new().scale(5, 5, 5);
    let c = Matrix::new().translate(10, 5, 7);

    let t = c * b * a;
    assert_eq!(t * p, Point::new(15, 0, 7));
}

#[test]
fn fluent_transformation_api() {
    let p = Point::new(1, 0, 1);

    let t = Matrix::new()
        .rotate_x(PI / 2.0)
        .scale(5, 5, 5)
        .translate(10, 5, 7);

    assert_eq!(t * p, Point::new(15, 0, 7));
}
