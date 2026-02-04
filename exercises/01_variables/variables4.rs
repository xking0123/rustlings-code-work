// TODO: Fix the compiler error.
fn main() {
    let mut x = 3; //mutabiity makes it work, allows values of a variable to be reassigned later instead of leaving it locked
    //as one value
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}