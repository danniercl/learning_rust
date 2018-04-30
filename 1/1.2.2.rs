use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Let's the Binary implemention for Point2D
impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let integer_x : i64 = self.x as i64;
        let integer_y : i64 = self.y as i64;

        let decimal_x : f64 = self.x - (integer_x as f64);
        let decimal_y : f64 = self.y - (integer_y as f64);

        write!(f,
               "x: {:b}.",
               integer_x)?;

        let mut temp_decimal : f64 = 0.0;
        let two : f64 = 2.0;
        let mut bit = "0";
        for x in 1..53 {
            temp_decimal += two.powi(-x);
            if temp_decimal > decimal_x {
                bit = "0";
                temp_decimal -= two.powi(-x);
            } else {
                bit = "1";
            }

            write!(f,
               "{}",
               bit)?;
        }

        write!(f,
        ", y: {:b}.",
        integer_y)?;

        temp_decimal = 0.0;
        for x in 1..53 {
            temp_decimal += two.powi(-x);
            if temp_decimal > decimal_y {
                bit = "0";
                temp_decimal -= two.powi(-x);
            } else {
                bit = "1";
            }

            write!(f,
               "{}",
               bit)?;
        }

        write!(f,"")
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Similarly, implement for Complex
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{}  +{}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    println!("What does Point2D look like in binary: {:b}?", point);

    let complex_number = Complex {real: 3.3, imag: 7.2};

    println!("Compare Complex Number:");
    println!("Display: {}", complex_number);
    println!("Debug: {:?}", complex_number);
}
