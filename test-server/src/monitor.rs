use std::time::Duration;
use sysinfo::System;
use tokio::time::interval;

pub async fn monitor_memory() {
    let mut interval = interval(Duration::from_secs(30));
    let mut sys = System::new_all();

    loop {
        interval.tick().await;
        sys.refresh_all();

        let total_memory = sys.total_memory(); // KB
        let used_memory = sys.used_memory(); // KB

        log::info!(
            "Memory Usage - Total: {:.2} MB, Used: {:.2} MB, Free: {:.2} MB",
            total_memory as f64 / 1024.0,
            used_memory as f64 / 1024.0,
            (total_memory - used_memory) as f64 / 1024.0
        );
    }
}
