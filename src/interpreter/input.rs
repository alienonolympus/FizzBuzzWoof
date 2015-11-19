pub struct Instructions {
    start:i32,
    end:i32,
}

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

pub fn analyse() -> Instructions {
    let mut temp = read_code();
    let tempclone = temp.clone();
    temp.truncate(tempclone.len() - 1);

    let input:Vec<&str> = temp.split(' ').collect();

    let mut output = Instructions {
        start: 0,
        end: 0,
    };

    let start = input[0].to_string()
                .parse::<i32>()
                .unwrap_or(0);

    let end = input[1].to_string()
                .parse::<i32>()
                .unwrap_or(0);

    output.setvar(start, end);

    return output;

    /*
    return match input.len() {
        0 =>  {
            let output = Instructions {
                start: 0,
                end: 0,
            };

            output
        },

        1 => {
            let number = input[0]
                .parse::<i32>()
                .unwrap_or(0);

            let output = Instructions {
                start: number,
                end: number,
            };

            output
        },

        2 => {
            let number0 = input[0]
                .parse::<i32>()
                .unwrap_or(0);

            let number1 = input[1]
                .parse::<i32>()
                .unwrap_or(0);

            let output = Instructions {
                start: number0,
                end: number1,
            };

            output
        },

        _ => {
            let output = Instructions {
                start: 0,
                end: 0,
            };

            output
        },
    }
*/
}
