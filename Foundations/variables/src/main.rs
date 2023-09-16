fn greet(s: String) -> String { // 4. ownership of s will be returned
    println!("Hello {s}"); // 3. ownership of 'name' now called s belongs to greet
    s // 5. returning ownership to outer scope
}

fn greet_borrow(s: &String) {
    println!("Hello {s}"); // borrowed 'name' from main() but NOT allowed to change it
}

fn greet_weird(s: &mut String) {
    *s += " owo"; // borrowed 'name' from main() but allowed to change it
}

fn main() {
    let mut name = "Kesh".to_string(); // 1. main() owns name
    // let returned_string = greet(name); // 2. ownership of name given to greet()
    // greet(returned_string); // 6. ownership of name is returned back to main()
    // greet_borrow(&name) ; // main() still owns name but lends it to greet_borrow -- this borrow is immutable
    greet_weird(&mut name); // main() still owns name but lends it to greet_weird -- this borrow IS mutable i.e. greet_weird can change it
    println!("Hello {name}");
}
