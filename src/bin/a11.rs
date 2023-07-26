struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("id: {:?}", item.id);
}

fn main() {
    let my_grocery = GroceryItem {
        id: 1,
        quantity: 20,
    };

    display_quantity(&my_grocery);
    display_id(&my_grocery);
}
