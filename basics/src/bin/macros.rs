use std::{ fmt::Display, io };

// Declarative Macro!
macro_rules! sayhello {
    () => {
        println!("sayy hello bitchh !");
    };
}

// Procedural Macro | here Debug and Clone are the examples of Procedural Macro
#[derive(Debug, Clone)]
struct User {
    username: String,
    age: i32,
}

// Display here is the Trait that is being implemented on User Struct
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.username, self.age)
    }
}

impl User {
    fn print_username(&self) -> String {
        println!("username : {}", &self.username);

        // here we are able to use the self.clone() only because we have implemented the Clone Macro
        return self.clone().username;
    }
}

fn main() {
    let v = vec![1, 2, 43];
    sayhello!();

    let u1: User = User { username: String::from("saad"), age: 34 };

    println!("u1 : {}", u1);
    println!("u1 debug : {:?}", u1);
}
