//Tests for 3, 5 and 7s.
fn test4repeat(input:i32) -> bool {

    //See if the ostream has printed anything
    let mut printed = false;

    //Looping through input for 3, 5 and 7s.
    for ch in input.to_string().chars() {
        if ch == '3' {
            print!("Fizz ");
            printed = true;
        }
        if ch == '5' {
            print!("Buzz ");
            printed = true;
        }
        if ch == '7' {
            print!("Woof ");
            printed = true;
        }
    }

    //Helps the analyse function to figure out what to do
    return printed;
}

//Tests for multiples of 3, 5 and 7.
fn test4multiple(input:i32) -> bool {

    //See if the ostream has printed anything
    let mut printed = false;

    //Multiple tests
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

    //Helps the analyse function to figure out what to do
    return printed;
}

//Analyse this!
pub fn analyse(input:i32) {

    //2 variables to makesure both tests are run
    let evaluate1 = test4repeat(input.clone());
    let evaluate2 = test4multiple(input.clone());

    //If nothing has been printed, print!
    if  !(evaluate1 || evaluate2) {
        print!("{}", input);
    };

    //Now move us to a new line.
    println!("");
}
