/*
* Activity
* Recap: Add the fmt::Display trait to the Matrix struct in the above example, so that if you switch from printing the debug format {:?} to the display format {}, you see the following output:

* ( 1.1 1.2 )
* ( 2.1 2.2 )
* You may want to refer back to the example for print display.

* Add a transpose function using the reverse function as a template, which accepts a matrix as an argument, and returns a matrix in which two elements have been swapped. For example:

* println!("Matrix:\n{}", matrix);
* println!("Transpose:\n{}", transpose(matrix));
* Results in the output:


* Matrix:
* ( 1.1 1.2 )
* ( 2.1 2.2 )
* Transpose:
* ( 1.1 2.1 )
* ( 1.2 2.2 )
*/

use std::fmt::{Display, Formatter, Result};

// The following struct is for the activity.
struct Matrix(f32, f32, f32, f32);

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
