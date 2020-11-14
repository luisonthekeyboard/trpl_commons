use std::str::FromStr;
use std::io;

pub fn read_and_parse<T: FromStr>() -> Result<T, T::Err> {
    let mut to_as_str = String::new();

    io::stdin()
        .read_line(&mut to_as_str)
        .expect("Failed to read line");

    return to_as_str.trim().parse();
}