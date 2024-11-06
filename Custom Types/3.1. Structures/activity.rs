// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: left, y: top },
        bottom_right: Point {
            x: right,
            y: bottom,
        },
    } = rect;
    (right - left) * (bottom - top)
}

fn square(point: Point, offset: f32) -> Rectangle {
    let Point { x, y } = point;
    Rectangle {
        top_left: point,
        bottom_right: Point {
            x: x + offset,
            y: y + offset,
        },
    }
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 5.2,
        ..another_point
    };

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    println!("{}", rect_area(_rectangle));
    println!("{:?}", square(point, 5.0))
}
