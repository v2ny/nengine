use std::{fmt, io::Write, path::Path};
use colorize::*; // Importing the Colorize trait for colorization
use fancy_regex as regex;
use regex::Regex; // For bolding text within double quotes

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

// Display trait to print log levels in a friendly way
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

use std::fs::OpenOptions;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub file: &'static str,
    pub line: u32,
    pub timestamp: SystemTime,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String, file: &'static str, line: u32) -> Self {
        Self {
            level,
            message,
            file,
            line,
            timestamp: SystemTime::now(),
        }
    }

    // Extract file name from full file path
    fn file_name(&self) -> &str {
        Path::new(self.file)
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("unknown"))
            .to_str()
            .unwrap_or("unknown")
    }

    fn bold_double_quotes(message: &str) -> String {
		let re = Regex::new(r#""([^"]*)""#).unwrap();
		re.replace_all(message, |caps: &regex::Captures| {
			format!("\"{}\"", caps[1].to_string().bold()) // Convert capture to owned String
		})
		.to_string()
	}
}

#[derive(Debug, Clone)]
pub struct Logger {
    logs: Vec<LogEntry>, // Store log entries in memory
    log_file: String,
}

impl Logger {
    // Initialize the logger with a log file
    pub fn new(log_file: &str) -> Self {
        Self {
            logs: Vec::new(),
            log_file: log_file.to_string(),
        }
    }

    // Log a new entry, but avoid duplicates
    pub fn log(&mut self, level: LogLevel, message: &str, file: &'static str, line: u32) {
        // Create the log entry
        let entry = LogEntry::new(level.clone(), message.to_string(), file, line);
        
        // Check if the log entry already exists in memory
        if !self.logs.iter().any(|log| log.message == entry.message && log.level == entry.level) {
            // Add to logs
            self.logs.push(entry.clone());
            
            // Sort the logs by level and timestamp
            self.logs.sort_by(|a, b| a.level.cmp(&b.level).then(a.timestamp.cmp(&b.timestamp)));

            // Print the log to the terminal with color and file name extraction
            self.print_to_terminal(&entry);

            // Write the log to the file
            self.write_to_file(&entry).expect("Failed to write to log file");
        }
    }

    // Print a single log entry to the terminal with colors and bold formatting
    fn print_to_terminal(&self, entry: &LogEntry) {
		let colorized_level = match entry.level {
			LogLevel::Info => format!("[{}]", entry.level).blue(),
			LogLevel::Warning => format!("[{}]", entry.level).yellow(),
			LogLevel::Error => format!("[{}]", entry.level).red(),
		};
	
		let message = LogEntry::bold_double_quotes(&entry.message); // Bold anything in double quotes
		let file_name = entry.file_name(); // File name in magenta
	
		println!(
			"{} {} - {}:{}: {}",
			colorized_level,
			entry.timestamp
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs(),
			file_name,
			entry.line,
			message
		);
	}

    // Write a single log entry to the log file
    fn write_to_file(&self, entry: &LogEntry) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;

        writeln!(
            file,
            "[{}] {} - {}:{}: {}",
            entry.level,
            entry.timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            entry.file_name(),
            entry.line,
            entry.message
        )?;
        Ok(())
    }
}

#[macro_export]
macro_rules! log {
    // Pattern to handle single argument (basic logging)
    ($logger:expr, $level:expr, $msg:expr) => {
        $logger.log($level, $msg, file!(), line!());
    };

    // Pattern to handle multiple arguments using `format!` for formatting
    ($logger:expr, $level:expr, $($arg:tt)*) => {
        $logger.log($level, &format!($($arg)*), file!(), line!());
    };
}

pub fn gl_error_to_message(error_code: u32) -> String {
    let error_message = match error_code {
        gl::NO_ERROR => "No error",
        gl::INVALID_ENUM => "Invalid enum: An unacceptable value is specified for an enumerated argument.",
        gl::INVALID_VALUE => "Invalid value: A numeric argument is out of range.",
        gl::INVALID_OPERATION => "Invalid operation: The specified operation is not allowed in the current state.",
        gl::STACK_OVERFLOW => "Stack overflow: This command would cause a stack overflow.",
        gl::STACK_UNDERFLOW => "Stack underflow: This command would cause a stack underflow.",
        gl::OUT_OF_MEMORY => "Out of memory: There is not enough memory left to execute the command.",
        gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid framebuffer operation: The framebuffer object is not complete.",
        _ => "Unknown error code",
    };

	error_message.bold()
}