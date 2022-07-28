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
    let files = fs::read_dir("./src").unwrap();
    let mut count = 0;
    for file in files {
        let path = match file {
            Ok(file) => file,
            Err(err) => {
                panic!("Could not read file: {}", err)
            }
        };
        let content = fs::read_to_string(path.path()).unwrap();
        for _word in content.split_whitespace() {
            count += 1;
        }
    }
    println!("{:?}", count)
}
