mod log;

fn main() {
    let file_path = "path_to_your_log_file.log"; // Replace with your log file path
    
    match log::read_log_file(file_path) {
        Ok(lines) => {
            let log_entries: Vec<log::LogEntry> = lines.iter()
                .filter_map(|line| log::parse_log_entry(line))
                .collect();

            let counts = log::count_log_levels(&log_entries);
            log::generate_summary(counts);
        },
        Err(e) => eprintln!("Error reading log file: {}", e),
    }
}
