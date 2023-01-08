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
        let a = Point::new(4.3, -4.2, 3.1);

        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
    }

    #[test]
    fn creating_a_vector() {
        let a = Vector::new(4.3, -4.2, 3.1);

        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
    }

    #[test]
    fn adding_two_vectors() {
        let v1 = Vector::new(3, -2, 5);
        let v2 = Vector::new(-2, 3, 1);

        assert_eq!(v1 + v2, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn adding_vector_and_point() {
        let v = Vector::new(3, -2, 5);
        let p = Point::new(-2, 3, 1);

        assert_eq!(v + p, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn adding_point_and_vector() {
        let p = Vector::new(3, -2, 5);
        let v = Point::new(-2, 3, 1);

        assert_eq!(p + v, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3, 2, 1);
        let p2 = Point::new(5, 6, 7);

        assert_eq!(p1 - p2, Vector::new(-2, -4, -6));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Point::new(3, 2, 1);
        let v = Vector::new(5, 6, 7);

        assert_eq!(p - v, Point::new(-2, -4, -6));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3, 2, 1);
        let v2 = Vector::new(5, 6, 7);

        assert_eq!(v1 - v2, Vector::new(-2, -4, -6));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector::new(0, 0, 0);
        let v = Vector::new(1, -2, 3);

        assert_eq!(zero - v, Vector::new(-1, 2, -3));
    }

    #[test]
    fn negating_a_vector() {
        let a = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(-a, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let a = Vector::new(1, -2, 3);
        assert_eq!(a * 3.5, Vector::new(3.5, -7, 10.5));
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let a = Vector::new(1, -2, 3);
        assert_eq!(a * 0.5, Vector::new(0.5, -1, 1.5));
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let a = Vector::new(1, -2, 3);
        assert_eq!(a / 2, Vector::new(0.5, -1, 1.5));
    }

    #[test]
    fn computing_vector_magnitudes() {
        assert_eq!(Vector::new(1, 0, 0).mag(), 1.0);
        assert_eq!(Vector::new(0, 1, 0).mag(), 1.0);
        assert_eq!(Vector::new(0, 0, 1).mag(), 1.0);
        assert_eq!(Vector::new(1, 2, 3).mag(), (14.0 as f64).sqrt());
        assert_eq!(Vector::new(-1, -2, -3).mag(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0() {
        let v = Vector::new(4, 0, 0);
        assert_eq!(v.norm(), Vector::new(1, 0, 0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Vector::new(1, 2, 3);
        assert_eq!(v.norm(), Vector::new(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Vector::new(1, 2, 3);
        let norm = v.norm();
        assert_eq!(norm.mag(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = Vector::new(1, 2, 3);
        let b = Vector::new(2, 3, 4);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Vector::new(1, 2, 3);
        let b = Vector::new(2, 3, 4);
        assert_eq!(a.cross(b), Vector::new(-1, 2, -1));
        assert_eq!(b.cross(a), Vector::new(1, -2, 1));
    }
}
