#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

/* I am not sure how can I get the rectangle area using
 * only two points
fn rect_area(rect: Rectangle) > (u32) {
}
*/

struct Rectangle_info {
    w: f32,
    h: f32,
}
// Calculate the rectangle area
fn rect_area(rect: Rectangle_info) -> (f32){
    // Use destructure
    let Rectangle_info{w: x, h: y} = rect;

    x*y
}

// Give the square coordinates
fn square(point: Point, side: f32) -> (Rectangle){
    // Using destructuration
    let Point {x: a, y: b} = point;

    let rect_return = Rectangle {
        p1: Point {x: a, y: b},
        p2: Point {x: a + side, y: b + side}
    };

    rect_return
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let rect = Rectangle_info{
        w: 12.5,
        h: 454.66,
    };

    println!("Rectangle Area: {}", rect_area(rect));

    // Make a square
    let side = 5.0;
    let s_point = Point {x: 4.0, y: 6.0};
    println!("Square: {:?}", square(s_point, side));

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
