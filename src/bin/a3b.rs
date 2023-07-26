fn display_result(num: i32) {
    if num > 5 {
        println!("{} > 5", num);
    } else if num < 5 {
        println!("{} < 5", num);
    } else {
        println!("{} == 5", num);
    }
}
fn main() {
    display_result(5);
}
