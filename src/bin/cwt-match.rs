fn main() {
    let my_name = "Bill";

    match my_name {
        "AG" => println!("that is my name"),
        "Bob" => println!("nope not my name"),
        "Alice" => println!("hello alic"),
        _ => println!("Hi {} nice to meet you!", {my_name})
    }
}