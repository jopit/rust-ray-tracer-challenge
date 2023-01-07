pub trait Tuple {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn creating_a_point() {
        let a = point::new(4.3, -4.2, 3.1);

        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
    }

    #[test]
    fn creating_a_vector() {
        let a = vector::new(4.3, -4.2, 3.1);

        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
    }

    #[test]
    fn adding_two_vectors() {
        let v1 = vector::new(3, -2, 5);
        let v2 = vector::new(-2, 3, 1);

        assert_eq!(v1 + v2, vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn adding_vector_and_point() {
        let v = vector::new(3, -2, 5);
        let p = point::new(-2, 3, 1);

        assert_eq!(v + p, point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn adding_point_and_vector() {
        let p = vector::new(3, -2, 5);
        let v = point::new(-2, 3, 1);

        assert_eq!(p + v, point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point::new(3, 2, 1);
        let p2 = point::new(5, 6, 7);

        assert_eq!(p1 - p2, vector::new(-2, -4, -6));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point::new(3, 2, 1);
        let v = vector::new(5, 6, 7);

        assert_eq!(p - v, point::new(-2, -4, -6));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector::new(3, 2, 1);
        let v2 = vector::new(5, 6, 7);

        assert_eq!(v1 - v2, vector::new(-2, -4, -6));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = vector::new(0, 0, 0);
        let v = vector::new(1, -2, 3);

        assert_eq!(zero - v, vector::new(-1, 2, -3));
    }

    #[test]
    fn negating_a_vector() {
        let a = vector::new(1.0, -2.0, 3.0);

        assert_eq!(-a, vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let a = vector::new(1, -2, 3);
        assert_eq!(a * 3.5, vector::new(3.5, -7, 10.5));
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let a = vector::new(1, -2, 3);
        assert_eq!(a * 0.5, vector::new(0.5, -1, 1.5));
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let a = vector::new(1, -2, 3);
        assert_eq!(a / 2, vector::new(0.5, -1, 1.5));
    }

    #[test]
    fn computing_vector_magnitudes() {
        assert_eq!(vector::new(1, 0, 0).mag(), 1.0);
        assert_eq!(vector::new(0, 1, 0).mag(), 1.0);
        assert_eq!(vector::new(0, 0, 1).mag(), 1.0);
        assert_eq!(vector::new(1, 2, 3).mag(), (14.0 as f64).sqrt());
        assert_eq!(vector::new(-1, -2, -3).mag(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0() {
        let v = vector::new(4, 0, 0);
        assert_eq!(v.norm(), vector::new(1, 0, 0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = vector::new(1, 2, 3);
        assert_eq!(v.norm(), vector::new(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = vector::new(1, 2, 3);
        let norm = v.norm();
        assert_eq!(norm.mag(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = vector::new(1, 2, 3);
        let b = vector::new(2, 3, 4);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector::new(1, 2, 3);
        let b = vector::new(2, 3, 4);
        assert_eq!(a.cross(b), vector::new(-1, 2, -1));
        assert_eq!(b.cross(a), vector::new(1, -2, 1));
    }
}
