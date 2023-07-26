// fn main() {
//     // option 1 in creating vector
//     let my_nums = vec![1, 2, 3];

//     // option 2 in creating vector
//     let mut my_numbers = Vec::new();
//     my_numbers.push(1);
//     my_numbers.push(2);
//     my_numbers.push(3);
//     my_numbers.pop();
//     my_numbers.len();

//     for num in my_nums {
//         println!("{:?}", num);
//     }
// }

struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 88 },
        Test { score: 77 },
        Test { score: 93 },
    ];

    for test in my_scores {
        println!("score: {:?}", test.score);
    }
}
