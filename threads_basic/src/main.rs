fn thread_hello() {
    println!("Hello from thread!");
}

fn main() {
    println!("Hello from main thread!");

    let thread_handler = std::thread::spawn(thread_hello);
    thread_handler.join();
}
