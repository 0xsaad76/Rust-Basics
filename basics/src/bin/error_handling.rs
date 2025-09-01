use std::fs;

fn main() {
    let file_contents =
        fs::read_to_string("/home/spider/learnings/rust/basics/src/bin/examplea.txt");

    match file_contents {
        Ok(contents) => {
            println!("{}", contents);
        }
        Err(error) => {
            println!("{:?}", error);
            println!("error reading from the file provided !");
        }
    }

    println!("A BIG CODE!");

    // panic_example(); //any code after this functon wont run, because the function is calling the panic method
}

fn panic_example() {
    panic!("y0uu are panickedd");

    println!(
        "this code wont run because the thread was panicked/stopped out with the panic method"
    );
}
