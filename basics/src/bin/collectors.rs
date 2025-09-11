fn main() {
    println!("COLLECTORS : Vectors, Arrays, HashMaps & Iterators \n");
    vectors();
}

fn vectors() {
    fn num_to_evens(inputs: &mut Vec<u32>) {
        let mut i: usize = 0;
        while i < inputs.len() as usize {
            if inputs[i] % 2 != 0 {
                inputs.remove(i);
            } else {
                i += 1;
            }
        }

        print!("Evens : {:?} \n", inputs);
    }

    let mut nums = vec![1, 3, 5, 10];
    println!("nums : {:?}", nums);

    num_to_evens(&mut nums);
}
