extern crate spin_sleep_tokio;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        let exec_time = std::time::Instant::now();
        for _i in 0..1000 {
            spin_sleep_tokio::sleep(std::time::Duration::from_millis(1)).await;
        }
        println!("time: {}", exec_time.elapsed().as_secs_f64())
    })
    .await
    .unwrap();
}
