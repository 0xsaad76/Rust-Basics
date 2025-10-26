use std::{ collections::HashMap };

fn main() {
    println!("COLLECTORS : Vectors, Arrays, HashMaps & Iterators \n");
    vectors();
    iterator();
    hashmaps();
}

fn iterator() {
    let mut v1 = vec![2, 3, 4, 5];

    let mut v_iter = v1.iter_mut();

    while let Some(v) = v_iter.next() {
        println!("v : {}", v);
    }

    println!("v0: {:?}", v1);

    let v1_iter = v1.iter();

    // this is the example of an consuming iterator
    let sum: i32 = v1_iter.sum();

    // filterning out all the odds from the vectors and mapping them again converting them back to evens
    let iter_even = v1
        .into_iter()
        .filter(|x| x % 2 != 0)
        .map(|x| x * 2);

    // converts iterator back to the vector/any
    let v2: Vec<i32> = iter_even.collect();

    println!("v2 : {:?}", v2);
}

fn vectors() {
    fn num_to_evens(inputs: &mut Vec<u32>) {
        let mut i: usize = 0;
        while i < (inputs.len() as usize) {
            if inputs[i] % 2 != 0 {
                inputs.remove(i);
            } else {
                i += 1;
            }
        }

        // you can also solve this problem with the help of Idiomatic Rust
        // inputs.retain(|&x| x % 2 == 0);

        print!("Evens : {:?} \n", inputs);
    }

    let mut nums = vec![1, 3, 5, 10];
    println!("nums : {:?}", nums);

    num_to_evens(&mut nums);
}

fn hashmaps() {
    fn group_by_keys(inputs: Vec<(u32, String)>) -> HashMap<u32, String> {
        let mut users_hashmaps = HashMap::new();
        for (key, name) in inputs {
            users_hashmaps.insert(key, name);
        }

        return users_hashmaps;
    }

    let users = vec![(1, "Saad".to_string()), (2, String::from("abban"))];
    let users_hashmap = group_by_keys(users);

    println!("user 1 : {:?}", users_hashmap.get(&1));
}
