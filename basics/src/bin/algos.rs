fn main() {
    //  1. Problem 1
    println!("!!! Algo Problems !!!");
    let vec_nums: Vec<u32> = vec![1, 3, 4, 2, 10];
    let num_stats: NumberStats = count_numbers(&vec_nums);
    println!("Number Stats  : {:#?}", &num_stats);
    // 2. Problem 2
    let mut v1 = VendingMachine {
        item_count: 100,
        item_price: 10,
    };
// 
    let failed_underpaid_purchase = v1.purchase(2, 5);
    let result1 = match failed_underpaid_purchase {
        PurchaseStatus::Failure(reason) => reason,
        PurchaseStatus::Success(change) => change.to_string(),
    };
    println!("Less pay : {:?}", result1);

    let failed_overquantity_purchase = v1.purchase(120, 120);
    let result1 = match failed_overquantity_purchase {
        PurchaseStatus::Failure(reason) => reason,
        PurchaseStatus::Success(change) => change.to_string(),
    };
    println!("More Quantity : {:?}", result1);

    let success_purchase = v1.purchase(2, 80);
    let result1 = match success_purchase {
        PurchaseStatus::Failure(reason) => reason,
        PurchaseStatus::Success(change) => change.to_string(),
    };
    println!("Success Purchase : {:?}", result1);

    // 3. Problem 3
    let vec1 = vec![2, 4, 5, 7, 8];
    let vec2 = vec![1, 3, 7, 11];

    let result1 = find_first_even_number(&vec1);
    match result1 {
        Some(n) => println!("Found Even Number in vector : {}", n),
        None => println!("Even Number Not found in vector"),
    }
    let result2 = find_first_even_number(&vec2);
    match result2 {
        Some(n) => println!("Found Even Number in vector : {}", n),
        None => println!("Even Number Not found in vector"),
    }

    println!("-----------------------");

    // 4. Program 4

    let fib = fibonacci(10);
    println!("fib : {} ", fib);
}

// 1. The Number Count

#[derive(Debug)]
pub struct NumberStats {
    even_count: u32,
    odd_cound: u32,
}

pub fn count_numbers(vec_nums: &Vec<u32>) -> NumberStats {
    let mut evens: u32 = 0;
    let mut odds: u32 = 0;

    for n in vec_nums {
        if n % 2 == 0 {
            // this is called idiom
            evens += evens;
        } else {
            odds = odds + 1;
        }
    }

    NumberStats {
        even_count: (evens),
        odd_cound: (odds),
    }
}

// 2. The Simple Vending Machine
struct VendingMachine {
    item_count: u32,
    item_price: u32,
}

#[derive(Debug)]
enum PurchaseStatus {
    Success(u32), // when a purchase is successful, telling the user how much change they get back.
    Failure(String), // when a purchase fails, explaining why (e.g., "Not enough stock" or "Not enough money"
}

impl VendingMachine {
    fn purchase(&mut self, quantity: u32, money_inserted: u32) -> PurchaseStatus {
        if quantity > self.item_count {
            return PurchaseStatus::Failure("Out of Stock".to_string());
        } else if money_inserted < self.item_price * quantity {
            return PurchaseStatus::Failure("You are underpaying !".to_string());
        } else {
            self.item_count -= quantity;
            let change: u32 = money_inserted - (self.item_price * quantity);

            return PurchaseStatus::Success(change);
        }
    }
}

// 3. Find the first of its Kind

fn find_first_even_number(nums: &[i32]) -> Option<i32> {
    for n in nums {
        if n % 2 == 0 {
            return Some(*n as i32);
        }
    }

    return None;
}

// 4. Fibonacci Program
fn fibonacci(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return 1;
    }

    for _ in 0..(num - 2) {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}
