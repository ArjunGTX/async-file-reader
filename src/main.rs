use std::fs;
#[tokio::main]
async fn main() {
    async_file().await;
}

async fn async_file() {
    let handles = vec![
        tokio::spawn(async {
            read_file_1().await;
        }),
        tokio::spawn(async {
            read_file_2().await;
        }),
        tokio::spawn(async {
            get_count().await;
        }),
    ];
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn read_file_1() {
    println!("reading file 1");
    let content = read_file("./src/test1.txt").await;
    println!("{}", content);
}

async fn read_file_2() {
    println!("reading file 2");
    let content = read_file("./src/test2.txt").await;
    println!("{}", content);
}

async fn read_file(path: &str) -> String {
    let file = fs::read_to_string(path).unwrap();
    file
}

async fn get_count() {
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
    println!("{}", count);
}
