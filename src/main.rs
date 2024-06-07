
/**
 * By default, Rust has a set of items defined in the standard library that it brings into the scope of every program.
 * This set is called the prelude, and you can see everything in it in the standard library documentation.
 * If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.
 * Using the std::io library provides you with a number of useful features, including the ability to accept user input.
 */
// import the io module from std library
use std::io;
// the Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use rand::Rng;
// The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    // we call the rand::thread_rng function that gives us the particular random number generator we’re going to use
    // The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your guess.");
        // Next, we’ll create a variable to store the user input
        // This line creates a new variable named apples and binds it to the value 5. In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.
        // To make a variable mutable, we add mut before the variable name
        // ::new line indicates that new is an associated function of the String type
        let mut guess = String::new();
        // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
        io::stdin()
            // read_line is to take whatever the user types into standard input and append that into a string
            // the & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
            .read_line(&mut guess) // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. The purpose of these Result types is to encode error-handling information.
            .expect("Failed to read line");
        // this line is to shadow the previous value of guess with a new one. This feature is often used in situations in which you want to convert a value from one type to another type.
        //  The guess in the expression(right) refers to the original guess variable that contained the input as a string
        let guess: u32 = match guess.trim().parse() {
            // parse returns a Result type, which is an enum. Enums are types that can have a fixed set of values, and those values are called the enum’s variants which are OK and Err
            // proper error handling, if the input can be parsed as a number, parse will return a Result variant that has an Ok value that contains the number.
            // If parse is called on a string that isn’t a number, it will return an Err value. The expect method is one of Result’s error-handling methods which is to keep asking user to guess a number
            Ok(num) => num,
            Err(_) => continue,
        };


        // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
        // It takes a reference to whatever you want to compare with: here it’s comparing guess to secret_number.
        match guess.cmp(&secret_number) {
            // The cmp method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with.
            // The cmp method returns a variant of the Ordering enum, which is either Less, Greater, or Equal. These variants are used with the match expression to decide what to do next.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
