fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin read_line failed");
    input.trim().to_string()
}

fn main() {
    let input = read_input();
    println!("You typed: [{input}]");
}
