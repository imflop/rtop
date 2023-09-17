use std::{time::Duration, thread};

use sysinfo::{System, SystemExt, ProcessExt};


fn main() {
    let mut system = System::new();

    loop {
        system.refresh_all();

        println!("{:<5} {:<10} {:<30} {:<10}", "PID", "CPU%", "Name", "Status");

        for (pid, proc_) in system.processes() {
            println!(
                "{:<5} {:<10} {:<30} {:<10}",
                pid,
                proc_.cpu_usage(),
                proc_.name(),
                match proc_.status() {
                    sysinfo::ProcessStatus::Idle => "Running",
                    sysinfo::ProcessStatus::Run => "Run",
                    sysinfo::ProcessStatus::Sleep => "Sleep",
                    sysinfo::ProcessStatus::Stop => "Stopped",
                    sysinfo::ProcessStatus::Zombie => "Zombie",
                    sysinfo::ProcessStatus::Tracing => "Tracing",
                    sysinfo::ProcessStatus::Unknown(_) => "Unknown",
                    _ => "Unknown1",
                }
            );
        }
        thread::sleep(Duration::from_secs(22));
    }
}
