//Required modules
mod process;
mod input;

//Analyse using the function in process.rs
fn analyse(input:i32) {
    process::analyse(input);
}

//Reads the code (includes a prompt!)
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

//The driver that puts everything together
pub fn driver() {
    //Looping the prompt
    loop {
        //Gets input(s)
        let result = input::analyse();

        if result.extra == "Exit".to_string() {
            return;
        }

        //Loops through the list given by the struct
        for count in result.start()..result.end() + 1 {
            //Clone as i32
            let input = count.clone() as i32;

            //Do the Fizzing, Buzzing, and Barking
            analyse(input);
        }
    }
}
