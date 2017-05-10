extern crate itertools;
use itertools::Itertools;

fn to_pig_latin(input: &str) -> String {
    input.split_whitespace()
         .map(|word| {
             let (dups, after) = word.split_at(word.find(&['a', 'e', 'i', 'o', 'u'][..]).unwrap_or(0));
             after.to_owned() + dups + "ay"
         })
         .join(" ")
}

fn main() {
    use std::env;
    use std::io;

    if env::args().len() > 1 {
        if env::args().nth(1) == Some("-h".to_string()) {
            println!("Usage: {} [-h] <string1, string2, ...>", env::args().nth(0).unwrap());
            println!("\nIf no strings are passed as parameters, the input will be read from stdin");
            println!("\n\th\tdisplay this help dialog");
        }
        else {
            for arg in env::args() {
                println!("{}", to_pig_latin(&arg));
            }
        }
    }
    else {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => println!("{}", to_pig_latin(&input)),
            _ => println!("There was an error reading from the console!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pig_latin() {
        assert_eq!("ellohay", &to_pig_latin("hello".to_string()));
        assert_eq!("orldway", &to_pig_latin("world".to_string()));

        assert_eq!("ellohay orldway", &to_pig_latin("hello world".to_string()));
    }
}
