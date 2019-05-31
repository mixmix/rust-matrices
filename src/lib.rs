#[derive(Debug, PartialEq)]
struct Matrix {
    data: [[f32; 4]; 4]

    // data: [
    //     [f32; 4],
    //     [f32; 4],
    //     [f32; 4],
    //     [f32; 4],
    // ]

}
impl Default for Matrix {
    fn default () -> Self {
        Matrix{
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }

    }
}
impl Matrix {
    // "class" method
    fn identity () -> Matrix {
        Matrix::default()
    }

    // first arg being self means this is an instance method
    fn set_value (&mut self, row: usize, col: usize, value: f32) {
        self.data[row - 1][col - 1] = value;
    }
}

// get rid of
fn identity_transform () -> Matrix {
    Matrix::default()
}

fn multiply (a: &Matrix, b: &Matrix) -> Matrix {
    // let mut output: Matrix = Matrix::default();
    let mut output: Matrix = Matrix::identity();

    for row in a.data[0].iter() {

    }

    output
}

#[cfg(test)]
mod tests {
    use Matrix;
    use identity_transform;
    use multiply;

    #[test]
    fn unit_matrix_multiplication () {
        let i = Matrix::identity();

        assert_eq!(multiply(&i, &i), i);
    }

    #[test]
    fn set_value () {
        let mut actual = Matrix::identity();
        actual.set_value(2, 3, 2.0);

        let expected = Matrix{
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 2.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        assert_eq!(actual, expected);
    }

    // #[test]
    // yet to be implemented
    fn matrix_multiplication () {
        let a = Matrix{
            data: [
                [0.0, 1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };
        let b = Matrix{
            data: [
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 2.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        let expected = Matrix{
            data: [
                [0.0, 1.0, 0.0, 2.0],
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        assert_eq!(multiply(&a, &b), expected);
    }
}
