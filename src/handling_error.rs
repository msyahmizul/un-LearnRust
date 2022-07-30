use std::fs::File;
use std::io;
use std::io::Read;
pub fn main() {
    // let v = vec![1, 2, 3];

    // v[99];

    // let f = std::fs::File::open("hello.txt");
    
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
}

// it's like golang return val and err
// ? use to shortcut of match err operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
