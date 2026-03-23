//more array thingies...
//array sclices

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = [2, 3, 4];

        assert_eq!([2, 3, 4], nice_slice);
        //made an array called nice_slice and physically made it equal to what i'm asserting it to be equal to
        //make it equal to an array that consists of 2, 3, and 4
    }
}
