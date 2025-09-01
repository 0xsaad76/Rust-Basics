// This file covers memory management concepts like ownership, borrowing, and the stack vs. the heap.

fn main() {
    println!("*** Running Memory Management ***");
    ownership();
    borrowing();
    stack_and_heap();
}

// Demonstrates the ownership system in Rust.
fn ownership() {
    println!("\n--- Ownership ---");

    // `s1` is the owner of the String "hello".
    let s1 = String::from("hello");

    // `s2` takes ownership of the String from `s1`.
    // `s1` is no longer valid after this line.
    let s2 = s1;

    // This would cause a compile-time error because `s1` is no longer valid.
    // println!("s1 = {}", s1);

    println!("s2 = {}", s2);

    // To create a deep copy, use the `clone()` method.
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
}

// Demonstrates borrowing and references.
fn borrowing() {
    println!("\n--- Borrowing ---");

    let mut my_string = String::from("Hello, Rust!");

    // We pass an immutable reference to `my_string` to the function.
    // The function can read the value but cannot modify it.
    print_string(&my_string);

    // We pass a mutable reference to `my_string` to the function.
    // The function can modify the value.
    change_string(&mut my_string);

    // `my_string` is still valid here because we only borrowed it.
    println!("Original string after changes: {}", my_string);
}

// A function that takes an immutable reference to a String.
fn print_string(s: &String) {
    println!("Inside print_string function: {}", s);
}

// A function that takes a mutable reference to a String and modifies it.
fn change_string(s: &mut String) {
    s.push_str(" and goodbye!");
    println!("Inside change_string function: {}", s);
}

// Demonstrates the difference between the stack and the heap.
fn stack_and_heap() {
    println!("\n--- Stack and Heap ---");
    stack_fn();
    heap_fn();
    update_string_on_heap();
}

// This function allocates variables on the stack.
fn stack_fn() {
    // `a`, `b`, and `c` are stored on the stack because they have a fixed size.
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

// This function allocates memory on the heap.
fn heap_fn() {
    // `s1` and `s2` are String, which are stored on the heap because their size can change.
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

// This function shows how a String can grow and reallocate on the heap.
fn update_string_on_heap() {
    let mut s = String::from("Initial string");
    println!("\n--- Updating String on Heap ---");
    println!("Before update: {}", s);
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    // Appending text to the string might cause it to reallocate if it exceeds its capacity.
    s.push_str(" and some additional text");
    println!("After update: {}", s);
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
}