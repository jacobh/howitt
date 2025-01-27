use core::time;

fn main() {
    loop {
        println!("tick");
        std::thread::sleep(time::Duration::from_secs(60));
    }
}
