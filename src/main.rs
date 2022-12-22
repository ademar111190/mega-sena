use std::io;
use unidecode::unidecode;

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
                    },
                    _ => {
                        println!("number {} for input {}", calculate(&user_input), user_input);
                    },
                }
            },
            Err(msg) => println!("Failed to read the name {}", msg),
        }
    }
}

fn clean_up(name: &str) -> String {
    unidecode(name)
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
}

fn calculate(name: &str) -> u32 {
    let mut result: u32 = 0;
    for c in name.chars() {
        result = (result + (c as u32) - ('a' as u32) + 1) % 60;
    }
    return if result == 0 { 60 } else { result };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(calculate(&clean_up("a")), 1);
        assert_eq!(calculate(&clean_up("z")), 26);
        assert_eq!(calculate(&clean_up("az")), 27);
        assert_eq!(calculate(&clean_up("zz")), 52);
        assert_eq!(calculate(&clean_up("zzh")), 60);
        assert_eq!(calculate(&clean_up("zzi")), 1);
        // a = 1 d = 4 e = 5 m = 13 a = 1 r = 18
        // 1 + 4 + 5 + 13 + 1 + 18 = 42
        // 42 % 60 = 42
        assert_eq!(calculate(&clean_up("Ademar")), 42);
        // j = 10 o = 15 a = 1 o = 15 b = 2 a = 1 t = 20 i = 9 s = 19 t = 20 a = 1
        // 10 + 15 + 1 + 15 + 2 + 1 + 20 + 9 + 19 + 20 + 1 = 113
        // 113 % 60 = 53
        assert_eq!(calculate(&clean_up("João Batista")), 53);
    }
}
