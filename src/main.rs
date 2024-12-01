use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone, Copy, Default, Debug)]
struct Coord {
    x: f32,
    y: f32,
}

trait Shape {
    fn origin(&self) -> Coord;
    fn set_origin(&mut self, origin: Coord);
    fn get_area(&self) -> f32;
}

struct Circle {
    origin: Coord,
    radius: f32,
}
struct Rectangle {
    origin: Coord,
    side_a: f32,
    side_b: f32,
}
struct Triangle {
    origin: Coord,
    base: f32,
    height: f32,
}

impl Shape for Circle {
    fn origin(&self) -> Coord {
        self.origin
    }

    fn set_origin(&mut self, origin: Coord) {
        self.origin = origin;
    }

    fn get_area(&self) -> f32 {
        PI * self.radius.powi(2)
    }
}
impl Shape for Rectangle {
    fn origin(&self) -> Coord {
        self.origin
    }

    fn set_origin(&mut self, origin: Coord) {
        self.origin = origin;
    }

    fn get_area(&self) -> f32 {
        self.side_a * self.side_b
    }
}
impl Shape for Triangle {
    fn origin(&self) -> Coord {
        self.origin
    }

    fn set_origin(&mut self, origin: Coord) {
        self.origin = origin;
    }

    fn get_area(&self) -> f32 {
        0.5 * self.base * self.height
    }
}

type ShapeObject = Arc<Mutex<dyn Shape + Send + Sync>>;

struct Canvas {
    shapes: Vec<ShapeObject>,
}

impl Canvas {
    fn new() -> Self {
        Canvas { shapes: Vec::new() }
    }
    fn add(&mut self, shape: ShapeObject) {
        self.shapes.push(shape);
    }
    fn get(&self, index: usize) -> Option<&ShapeObject> {
        self.shapes.get(index)
    }
    fn remove(&mut self, index: usize) -> ShapeObject {
        self.shapes.remove(index)
    }
    fn get_area(&self, index: usize) -> Option<f32> {
        self.get(index).map(|s| s.lock().unwrap().get_area())
    }
    fn set_origin(&self, index: usize, origin: Coord) {
        if let Some(shape) = self.shapes.get(index) {
            shape.lock().unwrap().set_origin(origin);
        }
    }
}

fn main() {
    let circle = Arc::new(Mutex::new(Circle {
        origin: Coord::default(),
        radius: 5.0,
    })) as ShapeObject;

    let rectangle = Arc::new(Mutex::new(Rectangle {
        origin: Coord::default(),
        side_a: 2.0,
        side_b: 4.0,
    })) as ShapeObject;

    let triangle = Arc::new(Mutex::new(Triangle {
        origin: Coord::default(),
        base: 2.0,
        height: 4.0,
    })) as ShapeObject;

    let mut canvas = Canvas::new();
    canvas.add(circle);
    canvas.add(rectangle);
    canvas.add(triangle);

    println!(
        "rectangle origin: {:?}",
        canvas.get(1).unwrap().lock().unwrap().origin()
    );

    // Update origin of rectangle
    canvas.set_origin(1, Coord { x: 5.0, y: 5.0 });

    println!(
        "rectangle origin: {:?}",
        canvas.get(1).unwrap().lock().unwrap().origin()
    );

    // Increment origin of rectangle in multiple threads
    let canvas = Arc::new(canvas);
    thread::scope(|scope| {
        for _ in 0..10 {
            let canvas = canvas.clone();
            scope.spawn(move || {
                let Coord { x, y } = canvas.get(1).unwrap().lock().unwrap().origin();
                canvas.set_origin(1, Coord { x: x + 1.0, y });
            });
        }
    });

    println!(
        "rectangle origin: {:?}",
        canvas.get(1).unwrap().lock().unwrap().origin()
    );
}
