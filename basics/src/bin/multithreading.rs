use std::thread;
use std::time::Duration;

fn main() {
    // this code spawns a new thread on the core and runs the snippet code parallelly
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawn thread !");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // this awaits the spawn thread to finish before running any code on the main thread
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread !");
        thread::sleep(Duration::from_millis(1));
    }
}
