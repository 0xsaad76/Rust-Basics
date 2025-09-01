// This is a  file covers the basics of Rust, including variables, conditionals, loops, and functions.

fn main() {
    println!("*** Running Basics ***");
    variables();
    conditionals();
    loops();
    functions();
}

// Demonstrates basic variable declaration and printing.
fn variables() {
    println!("--- Variables ---");
    let greetings = String::from("Hello World !!!");
    println!("{}", greetings);

    // `chars()` returns an iterator over the characters of a string.
    // `nth(0)` gets the character at index 0.
    let char1: Option<char> = greetings.chars().nth(0);

    // `match` is used for pattern matching. It's similar to a switch statement.
    match char1 {
        Some(c) => println!("First character: {}", c),
        None => println!("No character found at that index"),
    }
}

// Demonstrates if/else if/else conditionals.
fn conditionals() {
    println!("--- Conditionals ---");
    let is_male = true;
    let age = 25;

    if is_male {
        println!("He is a male.");
    } else {
        println!("She is a female.");
    }

    if is_male && age > 18 {
        println!("He is an 18+ male.");
    }
}

// Demonstrates a `for` loop and calls a function within the loop.
fn loops() {
    println!(
        "
--- Loops ---"
    );
    println!("Running a for loop from 0 to 4:");
    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }

    let sentence = String::from("love is blind");
    let first_word = get_first_word(sentence); // Note: `sentence` is moved here.

    println!("First word of the sentence is: {}", first_word);
}

// Demonstrates a simple function that takes a String and returns a String.
fn functions() {
    println!("--- Functions ---");
    let sentence = String::from("This is a sentence.");
    let first_word = get_first_word(sentence); // Note: `sentence` is moved here as well.
    println!("First word from the functions example: {}", first_word);
}

// A function to get the first word of a sentence.
// Note: This function takes ownership of the String.
fn get_first_word(sentence: String) -> String {
    let mut answer = String::new();

    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        answer.push(char);
    }

    // note: if we are just writing the name of the return variable at the end of the function, it acts as the return answer;
    answer
}
