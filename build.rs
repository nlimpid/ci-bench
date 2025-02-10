use byte_unit::Byte;
use colored::*;
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut sys = System::new_all();
    sys.refresh_all();

    print_separator("System Information");

    // CPU Information
    print_cpu_info(&sys);

    // Memory Information
    print_memory_info(&sys);

    // Disk Information
    print_disk_info(&sys);
}

fn print_separator(title: &str) {
    println!("\n{}", "=".repeat(50).bright_blue());
    println!("{}", title.bright_yellow().bold());
    println!("{}", "=".repeat(50).bright_blue());
}

fn print_cpu_info(sys: &System) {
    println!("\n{}:", "CPU Information".cyan().bold());
    println!("  Logical cores: {}", num_cpus::get());
    println!("  Physical cores: {}", num_cpus::get_physical());

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    println!("  CPU Usage: {:.1}%", cpu_usage);

    if let Some(freq) = sys.global_cpu_info().frequency() {
        println!("  CPU Frequency: {} MHz", freq);
    }
}

fn print_memory_info(sys: &System) {
    println!("\n{}:", "Memory Information".cyan().bold());

    let total_mem = Byte::from_bytes(sys.total_memory() as u128);
    let used_mem = Byte::from_bytes((sys.total_memory() - sys.available_memory()) as u128);
    let total_swap = Byte::from_bytes(sys.total_swap() as u128);
    let used_swap = Byte::from_bytes((sys.total_swap() - sys.free_swap()) as u128);

    println!(
        "  Total Memory: {:.2}",
        total_mem.get_appropriate_unit(true)
    );
    println!("  Used Memory: {:.2}", used_mem.get_appropriate_unit(true));
    println!("  Total Swap: {:.2}", total_swap.get_appropriate_unit(true));
    println!("  Used Swap: {:.2}", used_swap.get_appropriate_unit(true));
}

fn print_disk_info(sys: &System) {
    println!("\n{}:", "Disk Information".cyan().bold());

    for disk in sys.disks() {
        let total = Byte::from_bytes(disk.total_space() as u128);
        let used = Byte::from_bytes((disk.total_space() - disk.available_space()) as u128);

        println!("  Mount point: {}", disk.mount_point().to_string_lossy());
        println!("    Total: {:.2}", total.get_appropriate_unit(true));
        println!("    Used: {:.2}", used.get_appropriate_unit(true));
        println!();
    }
}
