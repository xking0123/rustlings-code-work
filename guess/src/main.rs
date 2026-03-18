use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number... or else!");

    //generating random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //allow multiple guesses with looping
    loop{
        println!("take a swing at it y dontcha");

    //the value on immutable variables wont change
    
    //storing values with variables
    let mut guess = String::new(); //mutable - value can change

    //recieving user input
    io::stdin()
        .read_line(&mut guess) //& key allows 'argument' to be a reference
        //means that it allows multiple parts of code to access piece of data without storing it in memory over and over
        .expect("failurrreeee"); //handling potential failure

    //move to 'match' to handling an error rather than crashing on an error
    let guess:u32 = match guess.trim().parse() { //parse turns string into number :)
        //handling invalid input
        Ok(num) => num,
        Err(_) => continue,
    };

    //printing values
    println!("choice: {guess}");

    //comparing guess to secret number
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("nah too small"),
        Ordering::Greater => println!("nah too big"),
        Ordering::Equal => { 
            println!("JUST RIGHT :)");
            break; //quitting after correct guess
        } 
    }
    }
}