pub struct Instructions {
    pub start:i32;
    pub end:i32;
}

fn read_code() -> String {
    use std::io::{BufRead , Write};
    use std::io;

    print!("$");
    let mut input = String::new();

    let _ = io::stdout().flush();
    let stdin = io::stdin();

    stdin.lock().read_line(&mut input).unwrap();
    return input;
}

pub fn analyse() {
    let mut input = read_code().chars();

    for ch in input
}
