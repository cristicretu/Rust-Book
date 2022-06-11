#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, i32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;

        ((x1 - x2) * (y1 - y2)).abs()
    }
}

fn make_square(p: Point, w: f32) -> Rectangle {
    let Point { x: new_x, y: new_y } = p;
    Rectangle {
        top_left: p,
        bottom_right: Point {
            x: new_x + w,
            y: new_y - w,
        },
    }
}

fn main() {
    let name = String::from("Pieter");
    let age = 27;

    let pieter = Person { name, age };

    println!("{:?}", pieter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right: Point = Point { x: 5.3, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: top_x, y: top_y } = point;

    println!("point coordinates: ({}, {})", top_x, top_y);

    // let _rectangle = Rectangle {
    //     top_left: Point { x: top_x, y: top_y },
    //     bottom_right,
    // };

    let square = make_square(point, 1.2);

    let _rectangle = Rectangle {
        top_left: Point { x: 1.0, y: 2.0 },
        bottom_right: Point { x: 3.0, y: 0.0 },
    };

    println!("rectangle area: {:?}", _rectangle.area());

    let _unit = Unit;

    let pair = Pair(1, 2);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);
}
