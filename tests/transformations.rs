use std::f64::consts::{PI, SQRT_2};

use ray_tracer::*;

#[test]
fn multiplying_by_a_translation_matrix() {
    let transform = translate(5, -3, 2);
    let p = point::new(-3, 4, 5);

    assert_eq!(transform * p, point::new(2, 1, 7));
}

#[test]
fn multiplying_by_the_inverse_of_a_translation_matrix() {
    let transform = translate(5, -3, 2);
    let p = point::new(-3, 4, 5);

    assert_eq!(transform.inverse() * p, point::new(-8, 7, 3));
}

#[test]
fn translation_does_not_affect_vectors() {
    let transform = translate(5, -3, 2);
    let v = vector::new(-3, 4, 5);

    assert_eq!(transform * v, vector::new(-3, 4, 5));
}

#[test]
fn a_scaling_matrix_applied_to_a_point() {
    let transform = scale(2, 3, 4);
    let p = point::new(-4, 6, 8);

    assert_eq!(transform * p, point::new(-8, 18, 32));
}

#[test]
fn a_scaling_matrix_applied_to_a_vector() {
    let transform = scale(2, 3, 4);
    let v = vector::new(-4, 6, 8);

    assert_eq!(transform * v, vector::new(-8, 18, 32));
}

#[test]
fn multiplying_by_the_inverse_of_a_scaling_matrix() {
    let transform = scale(2, 3, 4);
    let v = vector::new(-4, 6, 8);

    assert_eq!(transform.inverse() * v, vector::new(-2, 2, 2));
}

#[test]
fn reflection_is_scaling_by_a_negative_value() {
    let transform = scale(-1, 1, 1);
    let p = point::new(2, 3, 4);

    assert_eq!(transform * p, point::new(-2, 3, 4));
}

#[test]
fn rotating_a_point_around_the_x_axis() {
    let p = point::new(0, 1, 0);
    let half_quarter = rotate_x(PI / 4.0);
    let full_quarter = rotate_x(PI / 2.0);

    assert_eq!(half_quarter * p, point::new(0, SQRT_2 / 2.0, SQRT_2 / 2.0));
    assert_eq!(full_quarter * p, point::new(0, 0, 1));
}

#[test]
fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
    let p = point::new(0, 1, 0);
    let half_quarter = rotate_x(PI / 4.0);

    assert_eq!(
        half_quarter.inverse() * p,
        point::new(0, SQRT_2 / 2.0, -SQRT_2 / 2.0)
    );
}

#[test]
fn rotating_a_point_around_the_y_axis() {
    let p = point::new(0, 0, 1);
    let half_quarter = rotate_y(PI / 4.0);
    let full_quarter = rotate_y(PI / 2.0);

    assert_eq!(half_quarter * p, point::new(SQRT_2 / 2.0, 0, SQRT_2 / 2.0));
    assert_eq!(full_quarter * p, point::new(1, 0, 0));
}

#[test]
fn rotating_a_point_around_the_z_axis() {
    let p = point::new(0, 1, 0);
    let half_quarter = rotate_z(PI / 4.0);
    let full_quarter = rotate_z(PI / 2.0);

    assert_eq!(half_quarter * p, point::new(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0));
    assert_eq!(full_quarter * p, point::new(-1, 0, 0));
}

#[test]
fn a_shearing_transformation_moves_x_in_proportion_to_y() {
    let transform = shear(1, 0, 0, 0, 0, 0);
    let p = point::new(2, 3, 4);

    assert_eq!(transform * p, point::new(5, 3, 4));
}

#[test]
fn a_shearing_transformation_moves_x_in_proportion_to_z() {
    let transform = shear(0, 1, 0, 0, 0, 0);
    let p = point::new(2, 3, 4);

    assert_eq!(transform * p, point::new(6, 3, 4));
}

#[test]
fn a_shearing_transformation_moves_y_in_proportion_to_x() {
    let transform = shear(0, 0, 1, 0, 0, 0);
    let p = point::new(2, 3, 4);

    assert_eq!(transform * p, point::new(2, 5, 4));
}

#[test]
fn a_shearing_transformation_moves_y_in_proportion_to_z() {
    let transform = shear(0, 0, 0, 1, 0, 0);
    let p = point::new(2, 3, 4);

    assert_eq!(transform * p, point::new(2, 7, 4));
}

#[test]
fn a_shearing_transformation_moves_z_in_proportion_to_x() {
    let transform = shear(0, 0, 0, 0, 1, 0);
    let p = point::new(2, 3, 4);

    assert_eq!(transform * p, point::new(2, 3, 6));
}

#[test]
fn a_shearing_transformation_moves_z_in_proportion_to_y() {
    let transform = shear(0, 0, 0, 0, 0, 1);
    let p = point::new(2, 3, 4);

    assert_eq!(transform * p, point::new(2, 3, 7));
}

#[test]
fn individual_transformations_are_applied_in_sequence() {
    let p = point::new(1, 0, 1);
    let a = rotate_x(PI / 2.0);
    let b = scale(5, 5, 5);
    let c = translate(10, 5, 7);

    // apply rotation first
    let p2 = a * p;
    assert_eq!(p2, point::new(1, -1, 0));

    // then apply scaling
    let p3 = b * p2;
    assert_eq!(p3, point::new(5, -5, 0));

    // then apply translation
    let p4 = c * p3;
    assert_eq!(p4, point::new(15, 0, 7));
}

#[test]
fn chained_transformations_must_be_applied_in_reverse_order() {
    let p = point::new(1, 0, 1);
    let a = rotate_x(PI / 2.0);
    let b = scale(5, 5, 5);
    let c = translate(10, 5, 7);

    let t = c * b * a;
    assert_eq!(t * p, point::new(15, 0, 7));
}

#[test]
fn fluent_transformation_api() {
    let p = point::new(1, 0, 1);

    let t = matrix::new()
        .rotate_x(PI / 2.0)
        .scale(5, 5, 5)
        .translate(10, 5, 7);

    assert_eq!(t * p, point::new(15, 0, 7));
}
