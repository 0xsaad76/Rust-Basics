// Define a generic struct `User` with two generic type parameters, `T` and `U`.
struct User<T, U> {
    // `name` and `city` are of type `T`.
    name: T,
    city: T,
    // `age` is of type `U`.
    age: U,
}

fn main() {
    //! Create an instance of `User` where `T` is `&str` and `U` is `i32`.
    let u1 = User {
        name: "saad",
        city: "bangalore",
        age: 58,
    };

    println!("User1 : ({} {} {})", u1.name, u1.city, u1.age);
}
