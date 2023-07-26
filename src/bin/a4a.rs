
fn get_truth(what_truth: bool) {
    match what_truth {
        true => println!("it's the truth!"),
        false => println!("nope you are lying!")
    }
}

fn main() {
    
    let what_truth = false;

    get_truth(what_truth)
}