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
        print!("Woof ");
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
    let evaluate1 = test4repeat(input.clone());
    let evaluate2 = test4multiple(input.clone());
    if  !(evaluate1 || evaluate2) {
        print!("{}", input);
    };
    println!("");
}
