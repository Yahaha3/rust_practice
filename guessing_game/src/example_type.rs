
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({}, {}) \n({}, {})", self.0, self.1, self.2, self.3)
        write!(f, "({}, {}))\n", self.0, self.1)?;
        write!(f, "({}, {}))", self.2, self.3)
    }
}

#[cfg(test)]
mod tests {
    use crate::example_common;
    use crate::example_type::{reverse, transpose, Matrix};

    #[test]
    fn test_tuple() {
        example_common::debug_pre("Tuple");
        let long_tuple = (1u8, 2u16, 3u32, 4u64,
                                                          -1i8, -2i16, -3i32, -4i64,
                                                          0.1f32, 0.2f64, 'a', true);
        

        let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

        println!("tuple_of_tuples: {:?}", tuple_of_tuples);
        
        let pair = (1, true);
        println!("the reserved pair is {:?}", reverse(pair));

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{}", matrix);
        println!("transpose matrix: {}", transpose(matrix));
        example_common::debug_post();
    }
}