use tokio::fs;
use tokio::io::AsyncReadExt;
#[tokio::main]
async fn main() {
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
    println!("Reading file 1");
    let content = read_file("./src/test1.txt").await;
    println!("{}", content);
}

async fn read_file_2() {
    println!("Reading file 2");
    let content = read_file("./src/test2.txt").await;
    println!("{}", content);
}

async fn read_file(path: &str) -> String {
    let mut f = fs::File::open(path).await.unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).await.unwrap();
    buffer
}

async fn get_count() {
    let mut folder = fs::read_dir("./src").await.unwrap();
    let mut count = 0;
    loop {
        if let Some(file) = folder.next_entry().await.unwrap() {
            let path = file.path();
            let content = fs::read_to_string(path).await.unwrap();
            for _word in content.split_whitespace() {
                count += 1;
            }
        } else {
            break;
        }
    }
    println!("Total words: {}", count);
}
