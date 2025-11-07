use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

fn main() {
    let u1 = User {
        username: "saad".to_string(),
        password: "safd234".to_string(),
    };

    // Serialization is converting the Rust DataStructures to the Standard Datastructure like String
    let serializit = serde_json::to_string(&u1).unwrap();

    // match serializit {
    //     Ok(s) => println!("{s}"),    
    //     Err(e) => {
    //         panic!("error : {:?}", e);
    //     }
    // }

    // Desirialization is converting String/Json to the Rust Datastructure
    let deserializeit: User = serde_json::from_str(&serializit).unwrap();

    println!("deserialized : {:?}", deserializeit);
}
