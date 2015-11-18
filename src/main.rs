mod process;

fn analyse(input:i32) {
    process::analyse(input);
}

fn main() {
    for count in 1..101 {
        let input = count.clone() as i32;
        analyse(input);
    }
}
