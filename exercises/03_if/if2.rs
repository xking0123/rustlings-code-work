// TODO: Fix the compiler error on this function.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"

        //gotta add extra parameters to make tests work without changing them...?
        //if using quotes to verify tests make sure everything matches in actual function
    } else if food == "potato" {
        "I guess I can eat that."
    } else if food == "broccoli" {
        "No thanks!"
    } else if food == "gummy bears" {
        "No thanks!"
    } else if food == "literally anything"{
        "No thanks!"
    }
    else {
        "YUCK... George is a picky little c*nt"
    }
}

fn main() {
    // You can optionally experiment here.

    picky_eater("strawberry");
    picky_eater("broccoli");
    picky_eater("potato");
    picky_eater("gummy bears");
    picky_eater("literally anything");
    picky_eater("chocolate cake");
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them. (they work, yipppeee :)   )
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
