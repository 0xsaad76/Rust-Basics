// trait is like a blueprint for the struct to follow(just like interface in solidity and typescript)
trait Utility {
    fn guess_height(&self) -> Result<f32, String> {
        return Ok(0.0);
    }
}

struct User {
    name: String,
    age: i32,
}

// if implementing the trait to the struct any function defined in trait must be implemented
impl Utility for User {
    fn guess_height(&self) -> Result<f32, String> {
        if self.age < 1 || self.age > 60 {
            return Err("Invalid Age".to_string());
        }
        if self.age < 10 {
            Ok(4.0)
        } else if self.age > 10 && self.age < 16 {
            Ok(6.0)
        } else {
            Ok(6.5)
        }
    }
}

// Trait Parameter : this simply means pass any parameter that is implementing the trait "Utility"
fn guess_weight(user: impl Utility) -> Result<f32, String> {
    let user_height = user.guess_height();

    match user_height {
        Ok(height) => {
            if height < 4.0 && height > 1.0 {
                Ok(20.0)
            } else if height > 5.0 && height < 7.0 {
                Ok(60.0)
            } else {
                Err("Coudn't Guess Your Weight".to_string())
            }
        }
        Err(err) => Err(err),
    }
}

fn main() {
    let user1: User = User { name: "Saad".to_string(), age: 21 };

    let user1_age = user1.guess_height();
    

    println!("user1 name : {} age : {}", user1.name, user1.age);
    println!("user1 height : {:?}", match user1_age {
        Ok(height) => height,
        Err(e) => panic!("Error : {}", e),
    });

    // a static User2 Struct
    struct User2;
    impl Utility for User2 {}

    // passing user1 as the parameter here because the user1 is implementing the trait "Utility"
    let user1_weight = guess_weight(user1);

    // we were able to pass User2 as the parameter because the User2 struct is also implementing the "Utility" trait with the default function returns
    let user2_weight = guess_weight(User2);

    println!("user1 weight : {:?}", user1_weight);
    println!("user2 weight : {:?}", user2_weight);
}
