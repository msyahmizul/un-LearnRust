pub fn run(){
    // basic print
    println!("Hello from the other side");

    // Basic formatting
    println!("Ha ha {}",1);

    // Positional Arguments
    println!("{name} like to play {haha}",name="name",haha="sdfdf");

    // Placeholder traits
    println!("Binary: {:b} Hex",10);

    //Placholder debug traits
    println!("{:?}",(12,true,"Hello"));
}