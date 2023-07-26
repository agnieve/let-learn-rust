struct Dimentions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimentions {
    fn print(&self) {
        println!(
            "Dimentions: {:?} x {:?} x {:?}",
            self.width, self.height, self.depth
        );
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Color: red"),
            Color::Green => println!("Color: green"),
            Color::Blue => println!("Color: blue"),
        }
    }
}

struct ShippingBox {
    dimensions: Dimentions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimentions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimension = Dimentions {
        width: 20.0,
        height: 40.0,
        depth: 10.0,
    };

    let small_box = ShippingBox::new(45.0, Color::Red, small_dimension);
    small_box.print();
}
