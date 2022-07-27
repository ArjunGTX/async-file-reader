use std::fs;
use std::thread;
fn main() {
    // async_file();
    word_count();
}

fn async_file() {
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

fn word_count() {
    let file = fs::read_to_string("./src/test1.txt").unwrap();
    let mut count = 0;
    for _word in file.split_whitespace() {
        count += 1;
    }
    println!("{}", count)
}
