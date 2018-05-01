use std::fmt::{self, Formatter, Display};
// use std::{mem};

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Transpose a 2x2 Matrix
fn transpose(input_matrix: Matrix) -> (Matrix){

    let mut return_matrix: Matrix = input_matrix;

    let temp_0_1 = return_matrix.1;
    return_matrix.1 = return_matrix.2;
    return_matrix.2 = temp_0_1;

    return_matrix
}

/** I was searching a way to iterate over the tuples-struct Matrix and
 * get a Display triat with parameters, :( but I couldn't
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Get the lenght of the structure
        let matrix_len = (mem::size_of::<Matrix>())/((mem::size_of::<f32>()));
        // Get the size of the square matrix
        let matrix_size = (matrix_len as f64).sqrt();

        let mut i = 0.0;
        for x in 0..matrix_len {
            if 0.0 == i {
                write!(f, "(")?;
            }

            write!(f, " {} ", self.x)?;

            if (matrix_size - 1.0) == i {
                write!(f, ")\n")?;
                i = -1.0;
            }

            i = i + 1.0;
        }

        write!(f, "")
    }
}*/

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        write!(f, "( {} {} )\n( {} {} )",
               self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("DISPLAY: \n{}", matrix);

    println!("TRANSPOSE:\n{}", transpose(matrix));

}
