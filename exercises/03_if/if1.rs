fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables

    if a == b {
        println!("{a}");
        return a;
    }  else if a > b{
        println!("{a}");
        return a;
    } else if a < b{
        println!("{b}");
        return b;    
    //for rust MUST have final else statement (needs a 'just in case' none of other conditions are present)
    } else{
        println!("0");
        return 0;
    }
}

fn main() {
    // You can optionally experiment here.

    //they da same
    bigger(10, 10);
    
    //a bigger
    bigger(20, 10);
    
    //b bigger
    bigger(10, 20);
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
