use ray_tracer::*;

fn matrix_from_str(spec: &str) -> Matrix {
    let trim_chars: &[_] = &[' ', '\t', '|'];

    let lines: Vec<_> = spec.trim().lines().collect();
    let mut data = matrix_sized(lines.len());

    for (row, line) in spec.trim().lines().enumerate() {
        let fields: Vec<_> = line.trim_matches(trim_chars).split('|').collect();
        for (col, num) in fields.iter().enumerate() {
            let val = match (*num).trim().parse::<f64>() {
                Ok(x) => x,
                Err(_) => panic!("error parsing value as f64: \"{}\"\n", (*num)),
            };
            data[row][col] = val;
        }
    }

    data
}

#[test]
fn constructing_and_inspecting_a_4x4_matrix() {
    let m = matrix_from_str(
        "
        |  1   |  2   |  3   |  4   |
        |  5.5 |  6.5 |  7.5 |  8.5 |
        |  9   | 10   | 11   | 12   |
        | 13.5 | 14.5 | 15.5 | 16.5 |
    ",
    );

    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][3], 4.0);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[2][2], 11.0);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][2], 15.5);
}

#[test]
fn a_2x2_matrix_ought_to_be_representable() {
    let m = matrix_from_str(
        "
        | -3 |  5 |
        |  1 | -2 | 
    ",
    );

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[0][1], 5.0);
    assert_eq!(m[1][0], 1.0);
    assert_eq!(m[1][1], -2.0);
}

#[test]
fn a_3x3_matrix_ought_to_be_representable() {
    let m = matrix_from_str(
        "
        | -3 |  5 |  0 |
        |  1 | -2 | -7 |
        |  0 |  1 |  1 |
    ",
    );

    assert_eq!(m[0][0], -3.0);
    assert_eq!(m[0][1], 5.0);
    assert_eq!(m[0][2], 0.0);
    assert_eq!(m[1][0], 1.0);
    assert_eq!(m[1][1], -2.0);
    assert_eq!(m[1][2], -7.0);
    assert_eq!(m[2][0], 0.0);
    assert_eq!(m[2][1], 1.0);
    assert_eq!(m[2][2], 1.0);
}

#[test]
fn matrix_equality_with_identical_matrices() {
    let a = matrix_from_str(
        "
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
        ",
    );

    let b = matrix_from_str(
        "
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
        ",
    );

    assert!(a == b)
}

#[test]
fn matrix_equality_with_different_matrices() {
    let a = matrix_from_str(
        "
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
        ",
    );

    let b = matrix_from_str(
        "
        | 2 | 3 | 4 | 5 |
        | 6 | 7 | 8 | 9 |
        | 8 | 7 | 6 | 5 |
        | 4 | 3 | 2 | 1 |
        ",
    );

    assert!(a != b);
}

#[test]
fn multiplying_two_matrices() {
    let a = matrix_from_str(
        "
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
        ",
    );

    let b = matrix_from_str(
        "
        | -2 | 1 | 2 |  3 |
        |  3 | 2 | 1 | -1 |
        |  4 | 3 | 6 |  5 |
        |  1 | 2 | 7 |  8 |
        ",
    );

    let c = matrix_from_str(
        "
        | 20|  22 |  50 |  48 |
        | 44|  54 | 114 | 108 |
        | 40|  58 | 110 | 102 |
        | 16|  26 |  46 |  42 |
        ",
    );

    assert_eq!(a * b, c);
}

#[test]
fn a_matrix_multiplied_by_a_vector() {
    let a = matrix_from_str(
        "
        | 1 | 2 | 3 | 4 |
        | 2 | 4 | 4 | 2 |
        | 8 | 6 | 4 | 1 |
        | 0 | 0 | 0 | 1 |
        ",
    );
    let v = vector(1, 2, 3);

    assert_eq!(a * v, vector(14, 22, 32));
}

#[test]
fn a_matrix_multiplied_by_a_point() {
    let a = matrix_from_str(
        "
        | 1 | 2 | 3 | 4 |
        | 2 | 4 | 4 | 2 |
        | 8 | 6 | 4 | 1 |
        | 0 | 0 | 0 | 1 |
        ",
    );
    let p = point(1, 2, 3);

    assert_eq!(a * p, point(18, 24, 33));
}

#[test]
fn multiplying_a_matrix_by_the_identity_matrix() {
    let a = matrix_from_str(
        "
        | 0 | 1 |  2 |  4 |
        | 1 | 2 |  4 |  8 |
        | 2 | 4 |  8 | 16 |
        | 4 | 8 | 16 | 32 |
        ",
    );
    let ident = matrix();

    assert_eq!(ident * a, a);
    assert_eq!(a * ident, a);
}

#[test]
fn multiplying_the_identity_matrix_by_a_vector() {
    let ident = matrix();
    let v = vector(1, 2, 3);

    assert_eq!(ident * v, vector(1, 2, 3))
}

#[test]
fn multiplying_the_identity_matrix_by_a_point() {
    let ident = matrix();
    let p = point(1, 2, 3);

    assert_eq!(ident * p, point(1, 2, 3))
}

#[test]
fn transposing_a_matrix() {
    let a = matrix_from_str(
        "
        | 0 | 9 | 3 | 0 |
        | 9 | 8 | 0 | 8 |
        | 1 | 8 | 5 | 3 |
        | 0 | 0 | 5 | 8 |
        ",
    );

    let expected = matrix_from_str(
        "
        | 0 | 9 | 1 | 0 |
        | 9 | 8 | 8 | 0 |
        | 3 | 0 | 5 | 5 |
        | 0 | 8 | 3 | 8 |
        ",
    );

    assert_eq!(a.transpose(), expected);
}

#[test]
fn transposing_the_identity_matrix() {
    let a = matrix().transpose();
    assert_eq!(a, matrix());
}

#[test]
fn calculating_the_determinant_of_a_2x2_matrix() {
    let a = matrix_from_str(
        "
        |  1 | 5 |
        | -3 | 2 |
        ",
    );

    assert_eq!(a.determinant(), 17.0);
}

#[test]
fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    let a = matrix_from_str(
        "
        |  1 | 5 |  0 |
        | -3 | 2 |  7 |
        |  0 | 6 | -3 |
        ",
    );

    let expected = matrix_from_str(
        "
        | -3 | 2 |
        |  0 | 6 |
        ",
    );

    assert_eq!(a.submatrix(0, 2), expected);
}

#[test]
fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    let a = matrix_from_str(
        "
        | -6 |  1 |  1 |  6 |
        | -8 |  5 |  8 |  6 |
        | -1 |  0 |  8 |  2 |
        | -7 |  1 | -1 |  1 |
        ",
    );

    let expected = matrix_from_str(
        "
        | -6 |  1 | 6 |
        | -8 |  8 | 6 |
        | -7 | -1 | 1 |
        ",
    );

    assert_eq!(a.submatrix(2, 1), expected);
}

#[test]
fn calculating_a_minor_of_a_3x3_matrix() {
    let a = matrix_from_str(
        "
        |  3 |  5 |  0 |
        |  2 | -1 | -7 |
        |  6 | -1 |  5 |
        ",
    );
    let b = a.submatrix(1, 0);

    assert_eq!(b.determinant(), 25.0);
    assert_eq!(a.minor(1, 0), 25.0)
}

#[test]
fn calculating_a_cofactor_of_a_3x3_matrix() {
    let a = matrix_from_str(
        "
        |  3 |  5 |  0 |
        |  2 | -1 | -7 |
        |  6 | -1 |  5 |
        ",
    );

    assert_eq!(a.minor(0, 0), -12.0);
    assert_eq!(a.cofactor(0, 0), -12.0);
    assert_eq!(a.minor(1, 0), 25.0);
    assert_eq!(a.cofactor(1, 0), -25.0);
}

#[test]
fn calculating_the_determinant_of_a_3x3_matrix() {
    let a = matrix_from_str(
        "
        |  1 |  2 |  6 |
        | -5 |  8 | -4 |
        |  2 |  6 |  4 |
        ",
    );

    assert_eq!(a.cofactor(0, 0), 56.0);
    assert_eq!(a.cofactor(0, 1), 12.0);
    assert_eq!(a.cofactor(0, 2), -46.0);
    assert_eq!(a.determinant(), -196.0);
}

#[test]
fn calculating_the_determinant_of_a_4x4_matrix() {
    let a = matrix_from_str(
        "
        | -2 | -8 |  3 |  5 |
        | -3 |  1 |  7 |  3 |
        |  1 |  2 | -9 |  6 |
        | -6 |  7 |  7 | -9 |
        ",
    );

    assert_eq!(a.cofactor(0, 0), 690.0);
    assert_eq!(a.cofactor(0, 1), 447.0);
    assert_eq!(a.cofactor(0, 2), 210.0);
    assert_eq!(a.cofactor(0, 3), 51.0);
    assert_eq!(a.determinant(), -4071.0);
}

#[test]
fn testing_an_invertible_matrix_for_invertibility() {
    let a = matrix_from_str(
        "
        |  6 |  4 |  4 |  4 |
        |  5 |  5 |  7 |  6 |
        |  4 | -9 |  3 | -7 |
        |  9 |  1 |  7 | -6 |
        ",
    );

    assert_eq!(a.determinant(), -2120.0);
    assert!(a.is_invertible());
}

#[test]
fn testing_a_noninvertible_matrix_for_invertibility() {
    let a = matrix_from_str(
        "
        | -4 |  2 | -2 | -3 |
        |  9 |  6 |  2 |  6 |
        |  0 | -5 |  1 | -5 |
        |  0 |  0 |  0 |  0 |
        ",
    );

    assert_eq!(a.determinant(), 0.0);
    assert!(!a.is_invertible());
}

#[test]
fn calculating_the_inverse_of_a_matrix() {
    let a = matrix_from_str(
        "
        | -5 |  2 |  6 | -8 |
        |  1 | -5 |  1 |  8 |
        |  7 |  7 | -6 | -7 |
        |  1 | -3 |  7 |  4 |
        ",
    );
    let b = a.inverse();

    assert_eq!(a.determinant(), 532.0);
    assert_eq!(a.cofactor(2, 3), -160.0);
    assert_eq!(b[3][2], -160.0 / 532.0);
    assert_eq!(a.cofactor(3, 2), 105.0);
    assert_eq!(b[2][3], 105.0 / 532.0);
    assert!(
        b == matrix_from_str(
            "
            |  0.21805 |  0.45113 |  0.24060 | -0.04511 |
            | -0.80827 | -1.45677 | -0.44361 |  0.52068 |
            | -0.07895 | -0.22368 | -0.05263 |  0.19737 |
            | -0.52256 | -0.81391 | -0.30075 |  0.30639 |
            "
        )
    );
}

#[test]
fn calculating_the_inverse_of_another_matrix() {
    let a = matrix_from_str(
        "
        |  8 | -5 |  9 |  2 |
        |  7 |  5 |  6 |  1 |
        | -6 |  0 |  9 |  6 |
        | -3 |  0 | -9 | -4 |
        ",
    );

    assert!(
        a.inverse()
            == matrix_from_str(
                "
                | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
                | -0.07692 |  0.12308 |  0.02564 |  0.03077 |
                |  0.35897 |  0.35897 |  0.43590 |  0.92308 |
                | -0.69231 | -0.69231 | -0.76923 | -1.92308 |
                "
            )
    );
}

#[test]
fn calculating_the_inverse_of_a_third_matrix() {
    let a = matrix_from_str(
        "
        |  9 |  3 |  0 |  9 |
        | -5 | -2 | -6 | -3 |
        | -4 |  9 |  6 |  4 |
        | -7 |  6 |  6 |  2 |
        ",
    );

    assert!(
        a.inverse()
            == matrix_from_str(
                "
                | -0.04074 | -0.07778 |  0.14444 | -0.22222 |
                | -0.07778 |  0.03333 |  0.36667 | -0.33333 |
                | -0.02901 | -0.14630 | -0.10926 |  0.12963 |
                |  0.17778 |  0.06667 | -0.26667 |  0.33333 |
                "
            )
    );
}

#[test]
fn multiplying_a_product_by_its_inverse() {
    let a = matrix_from_str(
        "
        |  3 | -9 |  7 |  3 |
        |  3 | -8 |  2 | -9 |
        | -4 |  4 |  4 |  1 |
        | -6 |  5 | -1 |  1 |
        ",
    );
    let b = matrix_from_str(
        "
        |  8 |  2 |  2 |  2 |
        |  3 | -1 |  7 |  0 |
        |  7 |  0 |  5 |  4 |
        |  6 | -2 |  0 |  5 |
        ",
    );
    let c = a * b;

    assert!(c * b.inverse() == a);
}
