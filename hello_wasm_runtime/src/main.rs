fn main() {
    println!("Hello from Rust compiled to WebAssembly (WASI)!");
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);
}