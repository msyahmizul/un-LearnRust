// Primitve str = Immutable fixed-length string
// String = growablem heap-allocated data structure

pub fn run() {
    let mut hello = String::from("Hello ");
    println!("{}", hello.len());

    hello.push_str("World");
    println!("{}", hello);

    println!("Capacity : {}",hello.capacity());

    


}
