pub fn main() {
    // let condition = true;

    // let number = if condition { 5 } else {12 };

    // println!("The value of number is: {}", number);
    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    //     println!("The result is {}", counter);
    // };

    // println!("The result is {}", result);

    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");
    //  for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!")
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    if let t = some_u8_value {
        println!("three {}", t);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
