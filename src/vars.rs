// Variable hold primitive dara or reference to data
// variable immutable

pub fn run() {
    let name = "Syahmi";
    let mut age = 10;
    println!("Ny name is {} {}", name, age);
    age = 37;

    println!("Ny name is {} {}", name, age);

    // Const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // addign multiple variable
    let (my_name, my_age) = ("Syahmi", 10);
    println!("{} {}", my_name, my_age);
}
