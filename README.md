# Rust 'Logrok' Log File Analyzer

## Overview

This Rust command-line tool is designed to analyze log files by counting the occurrences of different log levels (e.g., `INFO`, `WARN`, `ERROR`). It parses the log entries, identifies the log level, and generates a summary of log level frequencies. The tool can process logs in standard formats, making it useful for quickly understanding the distribution of log messages within a file.

## Features

- **Log Level Counting:** Counts occurrences of `INFO`, `WARN`, `ERROR`, and other log levels.
- **Log Entry Parsing:** Parses log entries based on a standard timestamp and log level format.
- **Summary Generation:** Outputs a summary of log level counts to the console.

## Usage

### Command-Line Arguments

- `--log <logfile>`: Specifies the path to the log file you want to analyze.

### Example

To run the tool and analyze a log file:

```sh
cargo run -- --log path_to_your_log_file.log
```

Replace `path_to_your_log_file.log` with the actual path to your log file.

### Output

After running the tool, you’ll see a summary like this:

```
Log Level Summary:
INFO: 150
WARN: 23
ERROR: 7
```

## Installation

1. Clone this repository:
   ```sh
   git clone https://github.com/konkked/logrok.git
   ```
2. Navigate to the project directory:
   ```sh
   cd logrok
   ```
3. Build the project:
   ```sh
   cargo build
   ```
4. Run the tool with your log file:
   ```sh
   cargo run -- --log path_to_your_log_file.log
   ```

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss any changes.

## License

This project is licensed under the MIT License.