use byte_unit::Byte;
use colored::*;
use std::time::{Duration, Instant};
use sysinfo::DiskExt;
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut sys = System::new_all();
    sys.refresh_all();

    print_title("System Information");

    // CPU Information
    print_cpu_info(&sys);

    // Memory Information
    print_memory_info(&sys);

    // Disk Information
    print_disk_info(&sys);

    // 添加性能基准测试
    run_benchmarks();

    // 在最后添加表格总结
    print_summary_table(&sys);
}

fn print_summary_table(sys: &System) {
    let cpu_score = bench_cpu();
    let (read_speed, write_speed) = bench_memory();
    let total_memory = Byte::from_bytes(sys.total_memory() as u128);
    let used_memory = Byte::from_bytes((sys.total_memory() - sys.available_memory()) as u128);
    let memory_usage = (used_memory.get_bytes() as f64 / total_memory.get_bytes() as f64) * 100.0;
    let cpu_usage: f32 =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    println!("cargo:warning=┌──────────────────────────────────────────────────────────────┐");
    println!("cargo:warning=│                      系统状态速览                           │");
    println!("cargo:warning=├────────────────┬─────────────────────────────────────────────┤");
    println!("cargo:warning=│ CPU使用率      │ {:<43.1}% │", cpu_usage);
    println!(
        "cargo:warning=│ CPU性能得分    │ {:<43.1} GFLOPS │",
        cpu_score
    );
    println!(
        "cargo:warning=│ CPU评级        │ {:<43} │",
        rate_cpu_performance(cpu_score)
    );
    println!("cargo:warning=├────────────────┼─────────────────────────────────────────────┤");
    println!("cargo:warning=│ 内存使用率     │ {:<43.1}% │", memory_usage);
    println!(
        "cargo:warning=│ 内存读取速度   │ {:<43.1} GB/s │",
        read_speed
    );
    println!(
        "cargo:warning=│ 内存写入速度   │ {:<43.1} GB/s │",
        write_speed
    );
    println!(
        "cargo:warning=│ 内存评级       │ {:<43} │",
        rate_memory_performance(read_speed)
    );
    println!("cargo:warning=└────────────────┴─────────────────────────────────────────────┘");
}

fn print_title(title: &str) {
    println!("cargo:warning=");
    println!("cargo:warning={}", "=".repeat(60));
    println!("cargo:warning={:^60}", title);
    println!("cargo:warning={}", "=".repeat(60));
}

fn print_cpu_info(sys: &System) {
    let mut content = Vec::new();
    content.push(format!("Total CPU Count: {}", sys.cpus().len()));
    content.push(format!("Physical Core Count: {}", num_cpus::get_physical()));
    content.push(String::new());

    for (i, cpu) in sys.cpus().iter().enumerate() {
        content.push(format!(
            "CPU {}: {} @ {:.2} GHz (Usage: {:.1}%)",
            i,
            cpu.name(),
            cpu.frequency() as f64 / 1000.0,
            cpu.cpu_usage()
        ));
    }

    let formatted = format_box("CPU Information", content);
    for line in formatted.lines() {
        println!("cargo:warning={}", line);
    }
}

fn print_memory_info(sys: &System) {
    let mut content = Vec::new();
    let total_memory = Byte::from_bytes(sys.total_memory() as u128);
    let used_memory = Byte::from_bytes((sys.total_memory() - sys.available_memory()) as u128);
    let total_swap = Byte::from_bytes(sys.total_swap() as u128);
    let used_swap = Byte::from_bytes((sys.total_swap() - sys.free_swap()) as u128);

    content.push(format!(
        "Memory: {}/{} ({:.1}% used)",
        used_memory.get_appropriate_unit(true),
        total_memory.get_appropriate_unit(true),
        (used_memory.get_bytes() as f64 / total_memory.get_bytes() as f64) * 100.0
    ));

    content.push(format!(
        "Swap: {}/{} ({:.1}% used)",
        used_swap.get_appropriate_unit(true),
        total_swap.get_appropriate_unit(true),
        (used_swap.get_bytes() as f64 / total_swap.get_bytes() as f64) * 100.0
    ));

    let formatted = format_box("Memory Information", content);
    for line in formatted.lines() {
        println!("cargo:warning={}", line);
    }
}

fn print_disk_info(sys: &System) {
    let mut content = Vec::new();

    for disk in sys.disks() {
        let total = Byte::from_bytes(disk.total_space() as u128);
        let used = Byte::from_bytes((disk.total_space() - disk.available_space()) as u128);
        let mount_point = disk.mount_point().to_string_lossy();

        content.push(format!(
            "Mount point: {} ({:?})",
            mount_point,
            disk.name().to_string_lossy()
        ));
        content.push(format!(
            "Space: {}/{} ({:.1}% used)",
            used.get_appropriate_unit(true),
            total.get_appropriate_unit(true),
            (used.get_bytes() as f64 / total.get_bytes() as f64) * 100.0
        ));
        content.push(format!("File system: {:?}", disk.file_system()));
        content.push(String::new());
    }

    let formatted = format_box("Disk Information", content);
    for line in formatted.lines() {
        println!("cargo:warning={}", line);
    }
}

fn format_box(title: &str, content: Vec<String>) -> String {
    let width = 60;
    let horizontal_line = "━".repeat(width);
    let mut result = String::new();

    result.push_str(&format!("┏{}┓\n", horizontal_line));
    result.push_str(&format!("┃{:^width$}┃\n", title, width = width));
    result.push_str(&format!("┣{}┫\n", horizontal_line));

    for line in content {
        result.push_str(&format!("┃ {:<width$}┃\n", line, width = width - 1));
    }

    result.push_str(&format!("┗{}┛\n", horizontal_line));
    result
}

fn run_benchmarks() {
    let mut content = Vec::new();

    // CPU 性能测试
    content.push(String::from("CPU Performance Test:"));
    let cpu_score = bench_cpu();
    content.push(format!("CPU Score: {:.2} GFLOPS", cpu_score));
    content.push(format!(
        "CPU Performance Rating: {}",
        rate_cpu_performance(cpu_score)
    ));
    content.push(String::new());

    // 内存性能测试
    content.push(String::from("Memory Performance Test:"));
    let (read_speed, write_speed) = bench_memory();
    content.push(format!("Memory Read Speed: {:.2} GB/s", read_speed));
    content.push(format!("Memory Write Speed: {:.2} GB/s", write_speed));
    content.push(format!(
        "Memory Performance Rating: {}",
        rate_memory_performance(read_speed)
    ));

    let formatted = format_box("Performance Benchmark", content);
    for line in formatted.lines() {
        println!("cargo:warning={}", line);
    }
}

fn bench_cpu() -> f64 {
    let start = Instant::now();
    let iterations = 100_000_000;
    let mut result = 0.0;

    // 简单的浮点运算测试
    for i in 0..iterations {
        result += (i as f64).sqrt().sin();
    }

    let duration = start.elapsed().as_secs_f64();
    // 估算 GFLOPS (每次迭代约 3 次浮点运算)
    (iterations as f64 * 3.0) / (duration * 1e9)
}

fn bench_memory() -> (f64, f64) {
    const BUFFER_SIZE: usize = 1024 * 1024 * 256; // 256MB
    let mut buffer = vec![0u8; BUFFER_SIZE];

    // 测试写入速度
    let start = Instant::now();
    for i in 0..BUFFER_SIZE {
        buffer[i] = (i % 256) as u8;
    }
    let write_duration = start.elapsed().as_secs_f64();
    let write_speed = BUFFER_SIZE as f64 / (write_duration * 1024.0 * 1024.0 * 1024.0);

    // 测试读取速度
    let mut sum = 0u64;
    let start = Instant::now();
    for &byte in buffer.iter() {
        sum = sum.wrapping_add(byte as u64);
    }
    let read_duration = start.elapsed().as_secs_f64();
    let read_speed = BUFFER_SIZE as f64 / (read_duration * 1024.0 * 1024.0 * 1024.0);

    (read_speed, write_speed)
}

fn rate_cpu_performance(gflops: f64) -> &'static str {
    match gflops {
        x if x > 100.0 => "Excellent (High-End CPU)",
        x if x > 50.0 => "Very Good (Modern CPU)",
        x if x > 20.0 => "Good (Average CPU)",
        x if x > 10.0 => "Fair (Older CPU)",
        _ => "Basic (Entry Level CPU)",
    }
}

fn rate_memory_performance(gb_per_sec: f64) -> &'static str {
    match gb_per_sec {
        x if x > 20.0 => "Excellent (DDR4/DDR5)",
        x if x > 15.0 => "Very Good (Fast DDR4)",
        x if x > 10.0 => "Good (Standard DDR4)",
        x if x > 5.0 => "Fair (DDR3)",
        _ => "Basic (Older Memory)",
    }
}
