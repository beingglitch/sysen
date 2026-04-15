use sysinfo::{System};
use std::{thread, time::Duration};

pub fn handle_status_command() {
    let mut sys = System::new_all();

    // First refresh
    sys.refresh_all();

    // Small delay (IMPORTANT for CPU accuracy)
    thread::sleep(Duration::from_millis(300));

    // Second refresh
    sys.refresh_all();

    // =====================
    // Collect data
    // =====================

    let cpu_usage = sys.global_cpu_usage();

    let total_mem = sys.total_memory() as f64;
    let used_mem = sys.used_memory() as f64;
    let memory_percent = (used_mem / total_mem) * 100.0;

    let total_swap = sys.total_swap() as f64;
    let used_swap = sys.used_swap() as f64;
    let swap_percent = if total_swap > 0.0 {
        (used_swap / total_swap) * 100.0
    } else {
        0.0
    };

    // =====================
    // Find top process
    // =====================

    let mut top_process = None;

    for (_pid, process) in sys.processes() {
        let name = process.name().to_string_lossy();

        // Skip self + zero usage processes
        if name == "sysen" || process.cpu_usage() <= 0.0 {
            continue;
        }

        match top_process {
            None => top_process = Some(process),
            Some(current_top) => {
                if process.cpu_usage() > current_top.cpu_usage() {
                    top_process = Some(process);
                }
            }
        }
    }

    // =====================
    // Determine system load
    // =====================

    let system_load = if cpu_usage > 80.0 || memory_percent > 80.0 {
        "High"
    } else if cpu_usage > 50.0 || memory_percent > 60.0 {
        "Moderate"
    } else {
        "Low"
    };

    // =====================
    // Output
    // =====================

    println!("\nSystem Status:\n");

    println!("CPU: {:.2}%", cpu_usage);
    println!("Memory: {:.2}%", memory_percent);
    println!("Swap: {:.2}%\n", swap_percent);

    match top_process {
        Some(p) => {
            println!(
                "Top Process: {} ({:.2}%)",
                p.name().to_string_lossy(),
                p.cpu_usage()
            );
        }
        None => println!("Top Process: None"),
    }

    println!("\nSystem Load: {}\n", system_load);
}