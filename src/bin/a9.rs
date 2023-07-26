fn my_tuple(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    let (x, y) = my_tuple(3, 4);

    println!("x is {:?}", x);

    if y > 5 {
        println!("y is greater than 5");
    } else if y < 5 {
        println!("y is less than 5");
    } else {
        println!("y is equal to 5");
    }
}
