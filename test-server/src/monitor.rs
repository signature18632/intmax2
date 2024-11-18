use std::time::Duration;
use sysinfo::System;
use tokio::time::interval;

pub async fn monitor_memory() {
    let mut interval = interval(Duration::from_secs(30));
    let mut sys = System::new_all();

    loop {
        interval.tick().await;
        sys.refresh_all();

        let total_memory = sys.total_memory() as f64 / (1 << 30) as f64; // GB
        let used_memory = sys.used_memory() as f64 / (1 << 30) as f64; // GB

        log::info!(
            "Memory Usage - Total: {:.2} GB, Used: {:.2} GB, Free: {:.2} GB",
            total_memory,
            used_memory,
            total_memory - used_memory
        );
    }
}
