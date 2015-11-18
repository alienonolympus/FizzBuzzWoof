fn test4repeat(input:i32) -> bool {
    let mut printed = false;
    if input - input % 10 == 30 {
        print!("Fizz ");
        printed = true;
    }
    if input % 10 == 3 {
        print!("Fizz ");
        printed = true;
    }
    if input - input % 10 == 50 {
        print!("Buzz ");
        printed = true;
    }
    if input % 10 == 5 {
        print!("Buzz ");
        printed = true;
    }
    if input - input % 10 == 70 {
        print!("Woof ");
        printed = true;
    }
    if input % 10 == 7 {
        print!("YEAH ");
        printed = true;
    }

    return printed;
}

fn test4multiple(input:i32) -> bool {
    let mut printed = false;
    if input % 3 == 0 {
        print!("Fizz ");
        printed = true;
    }
    if input % 5 == 0 {
        print!("Buzz ");
        printed = true;
    }
    if input % 7 == 0 {
        print!("Woof ");
        printed = true;
    }

    return printed;
}

pub fn analyse(input:i32) {
    if !(test4repeat(input.clone()) || test4multiple(input.clone())) {
        print!("{}", input);
    };
    println!("");
}
