use std::fs::File;
use std::{fs, io};
use std::error::Error;
use std::io::{ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    Ok(())
}


fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}