use std::path::Path;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

fn main() {
    let sysinfo = System::new_all();

    let cpus = sysinfo.cpus();

    let disks = sysinfo.disks();

    // Helper functions to extract specific data

    // Find the root fs and report this
    let (disk_bytes_total, disk_bytes_free) = disks
        .iter()
        .find(|disk| {
            disk.mount_point() == Path::new("/") || disk.mount_point() == Path::new("C:\\")
        })
        .map(|disk| (disk.total_space(), disk.available_space()))
        .unwrap_or_else(|| (0, 0));

    // Attempt to get the clock speed from the name of the CPU
    let cpu_frequency_from_name = cpus.iter().next().and_then(|cpu| {
        cpu.brand()
            .split_once("GHz")
            .and_then(|(result, _)| result.trim().rsplit_once(' '))
            .and_then(|(_, result)| result.parse::<f32>().ok())
    });

    let global_cpu_frequency = match cpu_frequency_from_name {
        Some(freq) => freq,
        None => {
            // Get the frequency from average measured frequencies
            let global_cpu_frequency: f32 =
                cpus.iter().map(|cpu| cpu.frequency()).sum::<u64>() as f32 / cpus.len() as f32;
            // Shift to ghz to 1dp
            (global_cpu_frequency / 100.0).round() / 10.0
        }
    };

    println!("CPUS....");
    for cpu in cpus.iter() {
        println!("{}", cpu.frequency());
        println!("{}", cpu.name());
        println!("{}", cpu.vendor_id());
        println!("{}", cpu.brand());
    }
    println!("Disks....");
    for disk in disks.iter() {
        println!("{}", disk.mount_point().display());
    }
    println!("Aggregates");
    println!("Disk totals: {} {}", disk_bytes_total, disk_bytes_free);
    println!("Global freq: {}", global_cpu_frequency);
}
