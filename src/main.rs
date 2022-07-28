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
