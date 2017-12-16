
#[macro_use]
extern crate clap;
use clap::App;

use std::path::Path;
use std::fs::File;
use std::io;

/// Convert a single word to pig latin.
fn to_pig_latin(word: &str) -> String {
    let (dups, after) = word.split_at(word.find(&['a', 'e', 'i', 'o', 'u'][..]).unwrap_or(0));
    after.to_owned() + dups + "ay"
}

/// Parse some impl Read until the end of input.
fn parse_input<R: io::Read>(input: &mut R) {

    // Create string for buffer
    let mut buff = String::new();

    // Read input to buff
    match input.read_to_string(&mut buff) {
        Ok(_) => {

            // For each word seperated by whitespace
            for word in buff.split_whitespace() {

                // Translate that word
                print!("{} ", to_pig_latin(word));
            }
        }
        Err(_) => {
            eprintln!("There was an error reading the input");
            std::process::exit(-1);
        }
    }
}

/// Application entry point
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Check for -f parameter
    if let Some(file) = matches.value_of("file") {

        // Check if the file exists
        let path = Path::new(file);
        if path.exists() && path.is_file() {

            // Try to read the file
            if let Ok(mut file) = File::open(path) {
                parse_input(&mut file);
            } else {
                eprintln!("The file could not be opened!");
                std::process::exit(-1);
            }
        } else {
            eprintln!("The file can not be read");
            std::process::exit(-1);
        }
    }
    // Read from arguments
    else if let Some(values) = matches.values_of("INPUT") {
        for value in values {
            print!("{} ", to_pig_latin(value));
        }
    }
    // Read from STDIN
    else {
        parse_input(&mut io::stdin());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pig_latin() {
        assert_eq!("ellohay", &to_pig_latin("hello"));
        assert_eq!("orldway", &to_pig_latin("world"));
        assert_eq!("eatay", &to_pig_latin("eat"));
    }
}
