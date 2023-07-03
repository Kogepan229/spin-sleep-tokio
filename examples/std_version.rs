fn main() {
    let exec_time = std::time::Instant::now();
    for _i in 0..1000 {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    println!("time: {}", exec_time.elapsed().as_secs_f64())
}
