fn print_output(is_greater_than_100: bool) {
    match is_greater_than_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let num = 99;
    let is_gt_100 = num > 100;
    print_output(is_gt_100);
}
