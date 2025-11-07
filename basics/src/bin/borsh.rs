use borsh::{ BorshDeserialize, BorshSerialize };

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    username: String,
    password: String,
    skills: Vec<String>,
}

fn main() {
    let original = User {
        username: "saad".to_string(),
        password: "fasf234".to_string(),
        skills: vec![String::from("Swimming"), String::from("Coding")],
    };

    let mut buffer: Vec<u8> = Vec::new();

    original.serialize(&mut buffer).unwrap();

    println!("serialized value : {:?}", buffer);

    // Desirialization
    let desirialized = User::try_from_slice(&buffer).expect("failed to desirialize");

    println!("Desirialized : {:?}", desirialized);
}
