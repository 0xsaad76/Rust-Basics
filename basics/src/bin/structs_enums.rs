pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
    Nice(u32),
    Naughty,
}

#[derive(Debug)]
pub struct Kid {
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
    name: String,
    niceness: Niceness,
}

// Move yesterday's function to an associated function in the struct
pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    if good_deeds == 0 && bad_deeds == 0 {
        return false;
    }

    let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
    let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

    let ratio = good_deeds / (good_deeds + bad_deeds);

    ratio >= 0.75
}

impl Kid {
    // Static Function : these functions can't be called with the class instances/objects
    // any function not containing the &self argument is a static class function
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let isnice_result = is_nice(good_deeds, bad_deeds);
        let nice_enum: Niceness;

        if isnice_result {
            nice_enum = Niceness::Nice(good_deeds);
        } else {
            nice_enum = Niceness::Naughty;
        }
 
        // Return a Kid instance
        return Kid {
            name: name,
            niceness: nice_enum,
        };
    }

    // this is not a static function since the argument contains the "&self"
    fn get_kids_name(&self) {
        println!("Kid's name : {}", self.name);
    }
}

fn main() {
    let kid_name = String::from("ram");
    let kid1 = Kid::new(kid_name, 20, 1);

    println!("Kid : {:?}", kid1);
    kid1.get_kids_name();
}
