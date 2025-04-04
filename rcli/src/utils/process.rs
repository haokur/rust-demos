use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use sysinfo::{Pid, Process, System};

use crate::utils::text;

pub fn with_ctrl_c_handler<F: FnOnce()>(main_logic: F, exit_message: Option<&str>) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let msg = exit_message.unwrap_or("progress is exit").to_string();
    ctrlc::set_handler(move || {
        if r.load(Ordering::SeqCst) {
            println!("\n {} Caught Ctrl-C, shutting down.", msg);
            std::process::exit(0);
        }
    })
    .expect("Error setting Ctrl-C handler");
    main_logic();
}

#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub pid: u32,
    pub port: u32,
}

// #[cfg(target_os = "windows")]
// fn get_process_info_by_port(port: u16) -> Option<(u32, String)> {
//     let output = Command::new("netstat")
//         .arg("-ano")
//         .output()
//         .expect("Failed to execute netstat");
//
//     if !output.status.success() {
//         eprintln!("Failed to execute netstat");
//         return None;
//     }
//
//     let output_str = String::from_utf8_lossy(&output.stdout);
//
//     for line in output_str.lines() {
//         // 解析 netstat 输出，找到对应的端口行
//         if line.contains(&format!(":{},", port)) {
//             let columns: Vec<&str> = line.split_whitespace().collect();
//             if let Some(pid_str) = columns.last() {
//                 if let Ok(pid) = pid_str.parse::<u32>() {
//                     // 查找对应的程序名，通常需要根据 PID 查找
//                     let process_name = get_process_name_by_pid(pid);
//                     return Some((pid, process_name));
//                 }
//             }
//         }
//     }
//
//     None
// }

// #[cfg(target_os = "linux")]
// fn get_process_info_by_port(port: u32) -> Vec<Program> {
//     let mut match_programs: Vec<Program> = vec![];
//
//     let output = Command::new("ss")
//         .arg("-tuln")
//         .output()
//         .expect("Failed to execute ss");
//
//     if !output.status.success() {
//         eprintln!("Failed to execute ss");
//         return match_programs;
//     }
//
//     let mut match_program_pids: Vec<&str> = vec![];
//     let output_str = String::from_utf8_lossy(&output.stdout);
//
//     for line in output_str.lines() {
//         if line.contains(&format!(":{} ", port)) {
//             let columns: Vec<&str> = line.split_whitespace().collect();
//             // 解析 PID 并获取程序名
//             if let Some(pid_str) = columns.get(5) {
//                 if let Ok(pid) = pid_str.parse::<u32>() {
//                     match_programs.push(Program {
//                         pid,
//                         port,
//                         name: columns.get(1).unwrap_or(&"").to_string(),
//                     })
//                 }
//             }
//         }
//     }
//
//     match_programs
// }

#[test]
fn test_get_process_info_by_port() {
    let process_info = get_process_info_by_port(5500);
    println!(
        "test_get_process_info_by_port result is {:#?}",
        process_info
    );
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn get_process_info_by_port(port: u32) -> Vec<Program> {
    let mut match_programs: Vec<Program> = vec![];
    let mut match_program_pids: Vec<&str> = vec![];

    let output = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{port}"))
        .output()
        .expect("Failed to execute lsof");

    if !output.status.success() {
        eprintln!("lsof output result is empty");
        return match_programs;
    }

    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        let columns: Vec<&str> = line.split_whitespace().collect();
        let command = columns[0];
        let pid = columns[1];

        if command.ne("COMMAND") {
            if !match_program_pids.contains(&pid) {
                match_program_pids.push(pid);
                match_programs.push(Program {
                    name: command.to_string(),
                    pid: pid.parse::<u32>().unwrap(),
                    port,
                })
            }
        }
    }

    match_programs
}

fn kill_progress_by_pid(pid: u32) -> Result<String, String> {
    let output = Command::new("kill")
        .arg(pid.to_string())
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        Err(format!(
            "Failed to kill pid {}: {}",
            pid,
            String::from_utf8_lossy(&output.stderr)
        ))
    } else {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub fn kill_programs(programs: Vec<Program>, with_kill_log: bool) {
    programs
        .into_iter()
        .for_each(|program| match kill_progress_by_pid(program.pid) {
            Ok(_) => {
                if with_kill_log {
                    println!(
                        "✅ Kill program is {}, pid is {}, port is {} success",
                        program.name, program.pid, program.port
                    );
                }
            }
            Err(err) => print!("❌ Kill program {} failed: {}", program.name, err),
        })
}

#[test]
fn test_get_matched_programs_by_name_or_port() {
    // 获取系统的所有运行的process
    let mut system = System::new_all();
    system.refresh_all();
    let all_process = system.processes();

    let query_names_and_ports = vec!["Google Chrome", "5500"];
    let matched_programs = get_matched_programs_by_name_or_port(query_names_and_ports, all_process);
    println!("{:?}", matched_programs);
}

// 获取所有匹配端口或名称的程序
fn get_matched_programs_by_name_or_port(
    name_or_ports: Vec<&str>,
    all_process: &HashMap<Pid, Process>,
) -> Vec<Program> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut matched_programs: Vec<Program> = vec![];

    name_or_ports.into_iter().for_each(|name_or_port| {
        if text::is_valid_port(name_or_port) {
            let port = name_or_port.parse::<u16>().unwrap();
            let match_port_programs = get_process_info_by_port(port.into());
            match_port_programs
                .into_iter()
                .for_each(|match_port_program| {
                    matched_programs.push(match_port_program);
                })
        } else {
            all_process.iter().for_each(|(pid, process)| {
                let process_name = process.name().to_string_lossy();
                if process_name.contains(name_or_port) {
                    matched_programs.push(Program {
                        name: process_name.to_string().clone(),
                        pid: pid.as_u32(),
                        port: 0,
                    })
                }
            })
        }
    });

    matched_programs
}

pub fn fetch_all_matched_program(name_or_ports: Vec<&str>) -> Vec<Program> {
    // 获取系统的所有运行的process
    let mut system = System::new_all();
    system.refresh_all();
    let all_process = system.processes();

    // 获取匹配的程序
    let matched_programs = get_matched_programs_by_name_or_port(name_or_ports, all_process);
    matched_programs
}

// 找到匹配的程序，并kill
#[allow(dead_code)]
pub fn fetch_match_program_and_kill(name_or_ports: Vec<&str>) {
    let matched_programs = fetch_all_matched_program(name_or_ports);
    kill_programs(matched_programs, true);
}
