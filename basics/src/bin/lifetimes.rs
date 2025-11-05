// 🧠 'a = lifetime parameter
// It tells Rust that str1, str2, and the return value all live at least as long as 'a.
//
// Means: return value can't outlive both inputs.
// If any input goes out of scope first, result becomes invalid.
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

fn main() {
    let result;

    let str1 = String::from("saad");
    let str2 = String::from("abban");

    // ✅ valid: both str1 and str2 live here
    result = longest(&str1, &str2);

    // ⚠️ if str2 was in a smaller scope, result would be invalid
    println!("longest string: {}", result);

    let user1_name = String::from("saad");

    // passing the valid scopped lifetime variable/ if the user1_name lifetime was small then the rust compiler would throw the lifetime error
    let user1 = User { name: &user1_name };

    print!("user1 name: {}", user1.name);
}

// Struct with Lifetimes

struct User<'a> {
    name: &'a str,
}
