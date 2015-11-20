//A struct to output instructions
pub struct Instructions {
    start:i32,
    end:i32,
}

//Implement functions (including access functions) for struct
impl Instructions {
    fn setvar(&mut self, first:i32, second:i32) {
        self.start = first;
        self.end = second;
    }
    pub fn start(&self) -> i32 {
        return self.start;
    }

    pub fn end(&self) -> i32 {
        return self.end;
    }
}

//Read the code from istream
fn read_code() -> String {
    //Using what we need
    use std::io::{BufRead , Write};
    use std::io;

    //Print prompt and initialise
    print!("$ ");
    let mut input = String::new();

    //Flush the ostream and create the istream
    let _ = io::stdout().flush();
    let stdin = io::stdin();

    //Reads the line...
    stdin.lock().read_line(&mut input).unwrap();

    //And return the output
    return input;
}

//Analyses the input into a Instructions struct
pub fn analyse() -> Instructions {

    //Read the code, and get rid rid of the '\n'
    let mut temp = read_code();
    let tempclone = temp.clone();
    temp.truncate(tempclone.len() - 1);

    //Our new vector of numbers (supposedly)
    let input:Vec<&str> = temp.split(' ').collect();

    //Create output structure
    let mut output = Instructions {
        start: 0,
        end: 0,
    };

    //Conversion of &str -> i32
    //Returns 0 if incorrect input
    let start = input[0].to_string()
                .parse::<i32>()
                .unwrap_or(0)
    let end = input[1].to_string()
                .parse::<i32>()
                .unwrap_or(0);

    //Safely set the variables
    output.setvar(start, end);

    //And return the data structure
    return output;
}
