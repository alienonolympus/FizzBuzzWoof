mod process;

fn analyse(input:i32) {
    process::analyse(input);
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

pub fn driver() {

    let mut userinput = read_code();
    let inputclone = userinput.clone();
    userinput.truncate(inputclone.len() - 1);
    let limit = userinput
        .parse::<i32>()
        .unwrap_or(25);

    for count in 1..limit + 1 {
        let input = count.clone() as i32;
        analyse(input);
    }
}
