use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct ProcessInfo {
    pid: usize,
    name: String,
    state: String,
    threads: usize,
}

fn parse_info(info: BufReader<File>) -> ProcessInfo {
    let mut pinfo = ProcessInfo {
        pid: 0,
        name: "null".into(),
        state: "null".into(),
        threads: 0,
    };
    for line in info.lines() {
        match line {
            Ok(key) => {
                let mut s = key.split(":");
                let keyy = s.next();
                let val = s.next();
                if let Some(k) = keyy {
                    if k == "Name" {
                        if let Some(v) = val {
                            pinfo.name = v.trim().into();
                        }
                    } else if k == "Pid" {
                        if let Some(v) = val {
                            let num = v.trim().parse::<usize>();
                            if let Ok(n) = num {
                                pinfo.pid = n;
                            }
                        }
                    } else if k == "State" {
                        if let Some(v) = val {
                            pinfo.state = v.trim().into();
                        }
                    } else if k == "Threads" {
                        if let Some(v) = val {
                            let num = v.trim().parse::<usize>();
                            if let Ok(n) = num {
                                pinfo.threads = n;
                            }
                        }
                    }
                }
            }
            Err(err) => println!("Failed to parse, {}", err),
        }
    }
    pinfo
}

fn print_pinfo(pinfo: ProcessInfo) {
    println!("Process Info");
    println!("Name:{}", pinfo.name);
    println!("Pid:{}", pinfo.pid);
    println!("State:{}", pinfo.state);
    println!("Threads:{}", pinfo.threads);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: provide a PID. Example: cargo run -- 1234");
        std::process::exit(1);
    }

    let pid = &args[1];
    let path = format!("/proc/{}/status", pid);

    println!("PID: {}", pid);
    println!("Path: {}", path);

    // Call File::open directly
    match File::open(&path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let pinfo = parse_info(reader);
            print_pinfo(pinfo);
        }
        Err(err) => {
            eprintln!("Failed to read {}: {}", path, err);
            eprintln!("Make sure the PID exists and you have permission to read it.");
        }
    }
}
