extern crate num_traits;
use std::ops::Add;
use num_traits::{Num};


#[derive(Default, Debug, PartialEq)]
struct Matrix<T> 
    where T: Default + Num + Copy 
{
    data: Vec<Vec<T>>


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

impl<T: Default + Num + Copy> Add for Matrix<T> {
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

impl<T: Default + Num + Copy> Matrix<T> {
    // "class" method
    fn identity (size: usize) -> Matrix<T> {
        let mut rows = vec![T::default(); size];
        let mut data = vec![rows; size];

        for (i, r) in data.iter_mut().enumerate(){
            for (j, c) in r.iter_mut().enumerate() {
                if i == j{
                    *c = T::one();
                } 
            }
        } 
        Matrix{data}
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
        let i = Matrix::identity(4);
        let j = Matrix::identity(4);
        let expected = Matrix{
            data: vec![
                vec![2.0, 0.0, 0.0, 0.0],
                vec![0.0, 2.0, 0.0, 0.0],
                vec![0.0, 0.0, 2.0, 0.0],
                vec![0.0, 0.0, 0.0, 2.0]
            ]
        };

        assert_eq!(i + j, expected);
    }

    #[test]
    fn identity(){
        let i = Matrix::identity(4);
        let expected = Matrix{
            data: vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0]
            ]
        };
        assert_eq!(i, expected);
    }

    // #[test]
    fn unit_matrix_multiplication () {
        let i = Matrix::<f32>::identity(4);

        // assert_eq!(multiply(&i, &i), i);
    }

    #[test]
    fn set_value () {
        let mut actual = Matrix::identity(4);
        actual.set_value(2, 3, 2.0);

        let expected = Matrix{
            data: vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 2.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0]
            ]
        };

        assert_eq!(actual, expected);
    }

    // #[test]
    // yet to be implemented
    fn matrix_multiplication () {
        let a = Matrix{
            data: vec![
                vec![0.0, 1.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0]
            ]
        };
        let b = Matrix{
            data: vec![
                vec![1.0, 0.0, 0.0, 1.0],
                vec![0.0, 1.0, 0.0, 2.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0]
            ]
        };

        let expected = Matrix{
            data: vec![
                vec![0.0, 1.0, 0.0, 2.0],
                vec![1.0, 0.0, 0.0, 1.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0]
            ]
        };

        // assert_eq!(multiply(&a, &b), expected);
    }
}
