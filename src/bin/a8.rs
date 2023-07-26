enum Flavor {
    Coffee,
    Tea,
    Water,
}

struct MyDrink {
    flavor: Flavor,
    ounce: i32,
}

fn my_flavor(my_flavor: Flavor) -> &'static str {
    match my_flavor {
        Flavor::Coffee => "Coffee",
        Flavor::Tea => "Tea",
        Flavor::Water => "Water",
    }
}

fn print_drink(drink: MyDrink) {
    println!("flavor: {}", my_flavor(drink.flavor));
    println!("ounce: {}", drink.ounce);
}

fn main() {
    let my_drink = MyDrink {
        flavor: Flavor::Coffee,
        ounce: 12,
    };

    print_drink(my_drink);
}
