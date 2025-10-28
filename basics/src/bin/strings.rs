fn main() {
    println!("Return the first name from full_name ! ");
    let name = String::from("saad abban");

    // this is the example for the string literal which literally means,the value is being stored the binary itself(during the compilation time)
    let s = "sdfa";

    let first = last_name(&name);

    println!("first name : {}", first);

    // slice example with the array example
    let arr = [1, 3, 4, 6, 7];

    let slice_arr = &arr[0..2];

    // yes, you can slice arrays, vectors, strings and anything that is stored on heap(maybe)
    println!("sliced array : {:?}", slice_arr);
}

fn last_name(fullname: &String) -> &str {
    let mut space_index = 0;

    for i in fullname.chars() {
        space_index = space_index + 1;

        if i == ' ' {
            break;
        }
    }

    // this function returns the slice(the reference/view to the actual value of String on heap) | not a copy.
    return &fullname[space_index..fullname.len()];
}
