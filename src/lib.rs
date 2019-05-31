extern crate num;
use std::ops::Add;

#[derive(Default, Debug, PartialEq)]
struct Matrix<T> 
    where T: Default + Add
{
    data: [[T; 4]; 4]

    // data: [
    //     [T; 4],
    //     [T; 4],
    //     [T; 4],
    //     [T; 4],
    // ]

}
// impl<T> Default for Matrix<T> {
//     fn default () -> Self {
//         Matrix::<T>{
//             data: [
//                 [1.0, 0.0, 0.0, 0.0],
//                 [0.0, 1.0, 0.0, 0.0],
//                 [0.0, 0.0, 1.0, 0.0],
//                 [0.0, 0.0, 0.0, 1.0]
//             ]
//         }
//     }
// }

impl<T: Default + Add> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: Self) -> Self {
        let mut result = Matrix::default();

        for (i, row) in self.data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                result.set_value(i + 1, j + 1, *col + other.data[i][j])
            }
        }

        result
    }
}

impl<T: Default + Add> Matrix<T> {
    // "class" method
    fn identity () -> Matrix<T> {
        Matrix::default()
    }

    // first arg being self means this is an instance method
    fn set_value (&mut self, row: usize, col: usize, value: T) {
        self.data[row - 1][col - 1] = value;
    }
}

// fn identity_transform () -> Matrix {
//     Matrix::default()
// }

// fn multiply (a: &Matrix, b: &Matrix) -> Matrix {
//     // let mut output: Matrix = Matrix::default();
//     let mut output: Matrix = Matrix::identity();

//     for row in a.data[0].iter() {

//     }

//     output
// }

#[cfg(test)]
mod tests {
    use Matrix;
    // use identity_transform;
    // use multiply;

    #[test]
    fn matrix_addition () {
        let i = Matrix::identity();
        let j = Matrix::identity();
        let expected = Matrix{
            data: [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 2.0]
            ]
        };

        assert_eq!(i + j, expected);
    }

    // #[test]
    fn unit_matrix_multiplication () {
        let i = Matrix::identity();

        // assert_eq!(multiply(&i, &i), i);
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

        // assert_eq!(multiply(&a, &b), expected);
    }
}
