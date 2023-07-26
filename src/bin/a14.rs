struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            age: 20,
            name: "John".to_owned(),
            color: "red".to_owned(),
        },
        Person {
            age: 9,
            name: "Nick".to_owned(),
            color: "green".to_owned(),
        },
        Person {
            age: 10,
            name: "AG".to_owned(),
            color: "blue".to_owned(),
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.color);
        }
    }
}
