use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

pub fn count_log_levels(log_entries: &[LogEntry]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    
    for entry in log_entries {
        *counts.entry(entry.level.clone()).or_insert(0) += 1;
    }
    
    counts
}

pub fn generate_summary(counts: HashMap<String, usize>) {
    println!("Log Level Summary:");
    for (level, count) in counts {
        println!("{}: {}", level, count);
    }
}

pub fn parse_log_entry(line: &str) -> Option<LogEntry> {
    let re = Regex::new(r"(?P<timestamp>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(?P<level>\w+)\] (?P<message>.*)").unwrap();
    
    re.captures(line).map(|cap| {
        LogEntry {
            timestamp: cap["timestamp"].to_string(),
            level: cap["level"].to_string(),
            message: cap["message"].to_string(),
        }
    })
}

pub fn read_log_file(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().filter_map(Result::ok).collect())
}