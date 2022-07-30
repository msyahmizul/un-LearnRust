use rand::Rng;
pub fn run() {
    println!("Guess the number!");

    println!("Input ypur guess.");

    let mut buf = String::new();

    std::io::stdin()
        .read_line(&mut buf)
        .expect("sdfgdfghdfjkhdfjkgh");

    println!("You guessed: {}", buf);
}

pub fn secret_game() {
    println!("Guess the number!");

    let mut run_count = 0;

    loop {
        println!("Please input your guess.");
        let secret_number = rand::thread_rng().gen_range(1, 101);

        let guess = String::from("12");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                println!("Run {}", run_count);
                break;
            }
        }
        run_count += 1;
    }
}
