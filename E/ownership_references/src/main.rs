// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut)]
use ownership_references::{inspect, change, eat, add};

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    // This way we only inspect a variable and FORBID to change it due to
    // IMMUTABLE reference to MUTABLE variable
    // We can access the initial variable later on
    inspect(&arg);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    // This way we change the initial value of a variable using MUTABLE reference to it
    change(&mut arg);
    println!("I have many {}", arg);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    // It consumes initial variable and do stuff with it. Later it will be impossible to access it
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "add" that takes *references* to two integer arguments,
    // dereferences them and adds them together, and returns the result.
    //
    // Here we pass ANY type (IMMUTABLE or MUTABLE) of reference, DEREFERENCE them in function
    // (ACCESSING THE INITIAL VARIABLES), adding them together, returning the value
    // keeping the initial variables available for later interactions
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    println!("1 + 2 = {}, even via references", add(&mut a, &mut b));
    println!("initial a: {} and b: {}", a, b);
}
