
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet
}

fn color_result(color: Color){
    match color {
        Color::Red => println!("Red!"),
        Color::Orange => println!("Orange!"),
        Color::Yellow => println!("Yellow!"),
        Color::Green => println!("Green!"),
        Color::Blue => println!("Blue!"),
        Color::Indigo => println!("Indigo!"),
        Color::Violet => println!("Violet!")
    }
}

fn main() {
    let mycolor = Color::Red;
    color_result(mycolor);
}