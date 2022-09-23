use rand::Rng;

pub fn start() {
    println!("Guess game start");

    let _rand_number = rand::thread_rng().gen_range(1..10);

    loop {
        println!("Please input a number");

        let mut _guess = String::new();
        std::io::stdin().read_line(&mut _guess).expect("数据错误");

        let _guess: u32 = match _guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The number you input is {}", _guess);
        println!("The rand number is {}", _rand_number);

        match _guess.cmp(&_rand_number) {
            std::cmp::Ordering::Less => println!("Too less"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
