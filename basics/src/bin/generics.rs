// Define a generic struct `User` with two generic type parameters, `T` and `U`.
struct User<T, U> {
    // `name` and `city` are of type `T`.
    name: T,
    city: T,
    // `age` is of type `U`.
    age: U,
}

// another example

// Without this PartialOrd Trait, Rust wouldn't know if the types you're passing can be compared using <, and your code wouldn't compile.
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b { b } else { a }
}

fn main() {
    //! Create an instance of `User` where `T` is `&str` and `U` is `i32`.
    let u1 = User {
        name: "saad",
        city: "bangalore",
        age: 58,
    };

    println!("User1 : ({} {} {})", u1.name, u1.city, u1.age);

    // we are able to pass any numbers and strings in the largest function is because they implement the PartialOrd(which is used for the comparing 2 things)
    let ex1 = largest(3, 10);

    println!("largest : {}", ex1);
}
