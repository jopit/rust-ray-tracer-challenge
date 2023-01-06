use crate::{feq, point, vector, Point, Vector};
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    data: [[f64; 4]; 4],
    size: usize,
}

pub fn matrix() -> Matrix {
    matrix_sized(4)
}

pub fn matrix_sized(size: usize) -> Matrix {
    Matrix {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
        size,
    }
}

impl Matrix {
    pub fn transpose(&self) -> Matrix {
        let mut result = matrix_sized(self.size);
        for row in 0..self.size {
            for col in 0..self.size {
                result.data[col][row] = self.data[row][col];
            }
        }
        result
    }

    pub fn determinant(&self) -> f64 {
        if self.size == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            let mut det = 0.0;
            for col in 0..self.size {
                det += self.data[0][col] * self.cofactor(0, col);
            }
            det
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut result = matrix_sized(self.size - 1);

        for r in 0..self.size {
            if r == row {
                continue;
            }
            for c in 0..self.size {
                if c == col {
                    continue;
                }
                let r1 = if r > row { r - 1 } else { r };
                let c1 = if c > col { c - 1 } else { c };
                result.data[r1][c1] = self.data[r][c];
            }
        }
        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn is_invertible(&self) -> bool {
        !feq(self.determinant(), 0.0)
    }

    pub fn inverse(&self) -> Matrix {
        if !self.is_invertible() {
            panic!("matrix is not invertible: {:?}", self);
        }

        // create a matrix that consists of the cofactors of each of the original elements
        // then transpose that matrix of cofactors
        // then divide each element by the determinant of the original matrix
        let det = self.determinant();
        let mut m2 = matrix_sized(self.size);
        for row in 0..self.size {
            for col in 0..self.size {
                let c = self.cofactor(row, col);

                // note that "col, row" here, instead of "row, col",
                // accomplishes the transpose operation
                m2.data[col][row] = c / det;
            }
        }
        m2
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let size = self.size;
        for row in 0..size {
            for col in 0..size {
                if !feq(self.data[row][col], other.data[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(
            self.size == rhs.size,
            "can't multiply matrices of unequal size"
        );
        assert!(self.size == 4, "can only multiply 4x4 matrices");

        let mut result = matrix();
        for row in 0..4 {
            for col in 0..4 {
                result.data[row][col] = self.data[row][0] * rhs.data[0][col]
                    + self.data[row][1] * rhs.data[1][col]
                    + self.data[row][2] * rhs.data[2][col]
                    + self.data[row][3] * rhs.data[3][col]
            }
        }

        result
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        assert!(self.size == 4, "can only multiply a 4x4 matrix by a vector",);

        let dot = |row: [f64; 4]| row[0] * rhs.x + row[1] * rhs.y + row[2] * rhs.z;
        vector(dot(self.data[0]), dot(self.data[1]), dot(self.data[2]))
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        assert!(self.size == 4, "can only multiply a 4x4 matrix by a vector",);

        let dot = |row: [f64; 4]| row[0] * rhs.x + row[1] * rhs.y + row[2] * rhs.z + row[3];
        point(dot(self.data[0]), dot(self.data[1]), dot(self.data[2]))
    }
}
