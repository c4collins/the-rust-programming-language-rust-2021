use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

pub fn main() -> Result<(), Box<dyn Error>> {
    // _burn();
    // _vec_too_small();
    // _open_file_that_does_not_exist();
    // _unwrap_file_that_does_not_exist();
    error_propagation()?;

    Ok(())
}

fn _burn() {
    panic!("Burn it down");
}

fn _vec_too_small() {
    let v = vec![1, 2, 3];
    v[99];
}

fn _open_file_that_does_not_exist() {
    let filename = "hello.txt";
    let file_result = File::open(filename);
    let file_contents = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };
    println!("{:?}", file_contents);
}

fn _unwrap_file_that_does_not_exist() {
    let filename = "hello2.txt";
    // let file_result = File::open(filename).unwrap();

    let error_msg = format!("{} should be included in this project", filename);
    let file_result = File::open(filename).expect(error_msg.as_str());
    println!("{:?}", file_result);
}

fn error_propagation() -> Result<String, io::Error> {
    // let username = _read_username_from_file().expect("Username not found");
    // let username = _shorter_read_username_from_file().expect("Username not found");
    // let username =
    //     _even_shorter_read_username_from_file().expect("unable to get username from file");
    let username = file_to_string_shortcut()?;
    println!("{}", username);
    Ok(username)
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let filename = "hello2.txt";
    let username_file_result = File::open(filename);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn _shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello2.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _even_shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello2.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn file_to_string_shortcut() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
