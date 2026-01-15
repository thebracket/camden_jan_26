async fn one() {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("hello one");
}

async fn two() {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("hello two");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(one());
    tokio::join!(one(), two());
}
