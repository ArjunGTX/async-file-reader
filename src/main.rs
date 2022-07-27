#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        read_file_1().await;
        read_file_2().await
    });
    handle.await.unwrap();
}

async fn read_file_1() {
    let result = tokio::fs::read_to_string("./src/test1.txt").await;
    let content = result.unwrap();
    println!("{}", content)
}

async fn read_file_2() {
    let result = tokio::fs::read_to_string("./src/test2.txt").await;
    let content = result.unwrap();
    println!("{}", content)
}
