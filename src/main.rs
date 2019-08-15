use std::io;
use rand::Rng;

enum Action {
    Hit,
    Stand,
    DoubleDown,
    SplitCards,
    Surrender
}

fn parse_action(str: &String) -> Action {
    let result = match str.trim() {
        "hit" => Action::Hit,
        "stand" => Action::Stand,
        "double-down" => Action::DoubleDown,
        "split" => Action::SplitCards,
        "surrender" => Action::Surrender,
        _ => Action::Hit,
    };
    result
}

fn main() {
    println!("Play blackjack!");

    println!("Please input what you'd like to do (hit/stand/double-down/split/surrender):");

    let mut raw_action = String::new();

    let action = parse_action(&raw_action);

    let card = rand::thread_rng().gen_range(1, 101);

    io::stdin().read_line(&mut raw_action).expect("Failed to read line!");

    println!("You wanted to: {}", raw_action);
}
