use remotery_rs::{cpu_sample, Remotery, SampleFlags, Settings};

fn main() {
    let remotery = unsafe { Remotery::new(Settings::default()) }.unwrap();
    remotery.set_current_thread_name("CPU Example");
    let mut iter_count = 0;
    loop {
        cpu_sample!(remotery, "scope outer");
        let one_hundred_ms = std::time::Duration::from_millis(100);
        std::thread::sleep(one_hundred_ms);
        {
            cpu_sample!(remotery, "scope 1");
            let ten_ms = std::time::Duration::from_millis(10);
            std::thread::sleep(ten_ms);
        }
        {
            cpu_sample!(remotery, "scope 2");
            let twenty_ms = std::time::Duration::from_millis(20);
            std::thread::sleep(twenty_ms);
        }
        iter_count += 1;
        remotery.log_text(&format!("iter {}", iter_count))
    }
}
