// src/output.rs
use colored::*;

#[derive(Debug, Clone, Copy)] // Added Clone and Copy for easier passing
pub struct OutputConfig {
    pub verbose: bool,
    pub quiet: bool,
}

impl OutputConfig {
    pub fn new(verbose: bool, quiet: bool) -> Self {
        let effective_quiet = quiet;
        let effective_verbose = if effective_quiet { false } else { verbose };

        OutputConfig {
            verbose: effective_verbose,
            quiet: effective_quiet,
        }
    }

    /// For messages that only show in verbose mode (typically to stderr).
    /// These are silenced by quiet mode.
    pub fn verbose<S: AsRef<str>>(&self, message: S) {
        if self.verbose && !self.quiet { // verbose implies not quiet
            eprintln!("{}", message.as_ref().dimmed());
        }
    }

    /// For standard informational messages (not primary data output, typically to stdout).
    /// Silenced by quiet mode.
    pub fn info<S: AsRef<str>>(&self, message: S) {
        if !self.quiet {
            println!("{}", message.as_ref());
        }
    }

    /// For success messages (typically to stdout).
    /// Silenced by quiet mode.
    pub fn success<S: AsRef<str>>(&self, message: S) {
        if !self.quiet {
            println!("{}", message.as_ref().green());
        }
    }

    /// For primary data output that should always go to stdout (e.g., list items, show content, path).
    /// This is NOT silenced by the quiet flag.
    pub fn data<S: AsRef<str>>(&self, message: S) {
        println!("{}", message.as_ref());
    }

    /// For primary data output without a trailing newline.
    /// NOT silenced by the quiet flag.
    pub fn data_no_nl<S: AsRef<str>>(&self, message: S) {
        print!("{}", message.as_ref());
    }

    /// For warnings (typically to stderr).
    /// Silenced by quiet mode unless it's a critical warning.
    /// (For now, all warnings handled by this method are silenced by quiet).
    #[allow(dead_code)]
    pub fn warn<S: AsRef<str>>(&self, message: S) {
        if !self.quiet {
            eprintln!("{}", message.as_ref().yellow());
        }
    }

    // Critical errors are always printed to stderr by main.rs, so no method here.
}