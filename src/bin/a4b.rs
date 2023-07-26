
fn get_number(mynum: i32) {

    match mynum {
        1 => println!("one"),
        2 => println!("two"),
        3 => print!("three"),
        _ => println!("some other number")
    };
}

fn main() {
    
    let mynum = 5;

    get_number(mynum);
}