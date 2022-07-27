use std::fs;
use std::thread;
fn main() {
    let handle1 = thread::spawn(|| {
        println!("File one started");
        let file = fs::read_to_string("./src/test1.txt").unwrap();
        println!("{}", file);
    });
    let handle2 = thread::spawn(|| {
        println!("File two started");
        let file = fs::read_to_string("./src/test2.txt").unwrap();
        println!("{}", file);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
