use tokio::fs;
use tokio::task::JoinHandle;
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
    fs::read_to_string(path).await.unwrap()
}

async fn get_count() {
    let mut folder = fs::read_dir("./src").await.unwrap();
    let mut total = 0;
    let mut handles: Vec<JoinHandle<i32>> = vec![];
    loop {
        if let Some(file) = folder.next_entry().await.unwrap() {
            let path = file.path();
            handles.push(tokio::spawn(async {
                let content = fs::read_to_string(path).await.unwrap();
                let mut count = 0;
                for _word in content.split_whitespace() {
                    count += 1;
                }
                count
            }));
        } else {
            break;
        }
    }
    for handle in handles {
        total += handle.await.unwrap();
    }
    println!("Total words: {}", total);
}
