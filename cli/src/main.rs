use megasena::megasena::{calculate, clean_up};
use std::io;

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    loop {
        println!("Write the name to be calculate or exit to quit:");
        user_input.clear();
        match stdin.read_line(&mut user_input) {
            Ok(_) => {
                user_input = clean_up(&user_input);
                match user_input.as_str() {
                    "exit" => {
                        println!("Bye and good luck!");
                        break;
                    }
                    _ => {
                        println!("number {} for input {}", calculate(&user_input), user_input);
                    }
                }
            }
            Err(msg) => println!("Failed to read the name {}", msg),
        }
    }
}
