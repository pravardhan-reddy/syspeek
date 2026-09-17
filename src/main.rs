use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, fmt};

struct ProcessInfo {
    pid: usize,
    name: String,
    state: String,
    threads: usize,
}

#[derive(Debug)]
struct ProcessInfoError {
    message: String,
}

impl ProcessInfoError {
    pub fn new(str: &str) -> Self {
        Self {
            message: str.to_string(),
        }
    }
}
impl fmt::Display for ProcessInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ProcessInfoError {}

fn parse_info(info: BufReader<File>) -> Result<ProcessInfo, Box<dyn Error>> {
    let mut name: Option<String> = None;
    let mut pid: Option<usize> = None;
    let mut state: Option<String> = None;
    let mut threads: Option<usize> = None;
    for line in info.lines() {
        let k = line?;
        let mut t = k.split(":");
        let keyy = t.next();
        let val = t.next();
        let value = val.ok_or(ProcessInfoError::new("Value not found error"))?;
        match keyy {
            Some("Name") => {
                let nm = value.trim().into();
                name = Some(nm);
            }
            Some("Pid") => {
                if let Ok(n) = value.trim().parse::<usize>() {
                    pid = Some(n);
                }
            }
            Some("State") => {
                let st = value.trim().into();
                state = Some(st);
            }
            Some("Threads") => {
                if let Ok(n) = value.trim().parse::<usize>() {
                    threads = Some(n);
                }
            }
            Some(_) => {}
            None => {}
        }
    }
    match (name, pid, state, threads) {
        (Some(n), Some(p), Some(s), Some(t)) => {
            let pinfo = ProcessInfo {
                name: n,
                pid: p,
                state: s,
                threads: t,
            };
            Ok(pinfo)
        }
        _ => Err(Box::new(ProcessInfoError::new("Failed to parse info"))),
    }
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
            if let Ok(pinfo) = parse_info(reader) {
                print_pinfo(pinfo);
            } else {
                eprintln!("Error while parsing");
            }
        }
        Err(err) => {
            eprintln!("Failed to read {}: {}", path, err);
            eprintln!("Make sure the PID exists and you have permission to read it.");
        }
    }
}
