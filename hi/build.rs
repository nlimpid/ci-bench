use byte_unit::Byte;
use colored::*;
use sysinfo::DiskExt;
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

    // Force the output to be printed during build
    println!("cargo:warning=Build script execution completed");
    println!("cargo:warning=Build script is running!");
}

fn print_separator(title: &str) {
    println!("cargo:warning=\n{}", "=".repeat(50));
    println!("cargo:warning={}", title);
    println!("cargo:warning={}", "=".repeat(50));
}

fn print_cpu_info(sys: &System) {
    println!("cargo:warning=\nCPU Information:");
    println!("cargo:warning=CPU Count: {}", sys.cpus().len());
    println!(
        "cargo:warning=Physical Core Count: {}",
        num_cpus::get_physical()
    );

    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!(
            "cargo:warning=CPU {}: {} @ {:.2} GHz (Usage: {:.1}%)",
            i,
            cpu.name(),
            cpu.frequency() as f64 / 1000.0,
            cpu.cpu_usage()
        );
    }
}

fn print_memory_info(sys: &System) {
    println!("cargo:warning=\nMemory Information:");
    let total_memory = Byte::from_bytes(sys.total_memory() as u128);
    let used_memory = Byte::from_bytes((sys.total_memory() - sys.available_memory()) as u128);
    let total_swap = Byte::from_bytes(sys.total_swap() as u128);
    let used_swap = Byte::from_bytes((sys.total_swap() - sys.free_swap()) as u128);

    println!(
        "cargo:warning=Memory: {}/{} ({:.1}% used)",
        used_memory.get_appropriate_unit(true),
        total_memory.get_appropriate_unit(true),
        (used_memory.get_bytes() as f64 / total_memory.get_bytes() as f64) * 100.0
    );

    println!(
        "cargo:warning=Swap: {}/{} ({:.1}% used)",
        used_swap.get_appropriate_unit(true),
        total_swap.get_appropriate_unit(true),
        (used_swap.get_bytes() as f64 / total_swap.get_bytes() as f64) * 100.0
    );
}

fn print_disk_info(sys: &System) {
    println!("cargo:warning=\nDisk Information:");

    for disk in sys.disks() {
        let total = Byte::from_bytes(disk.total_space() as u128);
        let used = Byte::from_bytes((disk.total_space() - disk.available_space()) as u128);
        let mount_point = disk.mount_point().to_string_lossy();

        println!(
            "cargo:warning=Mount point: {} ({:?})",
            mount_point,
            disk.name().to_string_lossy()
        );
        println!(
            "cargo:warning=Space: {}/{} ({:.1}% used)",
            used.get_appropriate_unit(true),
            total.get_appropriate_unit(true),
            (used.get_bytes() as f64 / total.get_bytes() as f64) * 100.0
        );
        println!("cargo:warning=File system: {:?}", disk.file_system());
        println!("cargo:warning=");
    }
}
