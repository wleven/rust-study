// pub mod guess_game;

mod calc_area;
mod guess_game;

fn main() {
    let list = ["1. guess_game", "2. calc_area"];

    for item in list.iter() {
        println!("{}", item)
    }

    let mut test_id = String::new();

    println!("Please input test id");
    std::io::stdin().read_line(&mut test_id).expect("Err");

    let test_id: &str = test_id.trim();

    match test_id {
        "1" => guess_game::start(),

        "2" => {
            let area = calc_area::calc(&calc_area::React {
                width: 10,
                height: 2,
            });

            println!("The react area is {}", area)
        }

        _ => println!("Test id is invalid"),
    }
}
