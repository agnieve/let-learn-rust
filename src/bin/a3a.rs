
fn is_true(control: bool) {

    if control == true {
        println!("hello");
    }else{
        println!("goodbye");
    }
}

fn main() {
    
    let control = false;

    is_true(control);
}