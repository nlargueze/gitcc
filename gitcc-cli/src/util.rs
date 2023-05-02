//! Misc. utilities

/// Prints a new line to stderr
#[macro_export]
macro_rules! new_line {
    () => {
        eprintln!();
    };
}

/// Prints an info message to stderr
#[macro_export]
macro_rules! info {
    ($MSG:expr) => {{
        use colored::Colorize;
        eprintln!("{} {}", "i".blue().bold(), $MSG);
    }};
}

/// Prints a success message to stderr
#[macro_export]
macro_rules! success {
    ($MSG:expr) => {{
        use colored::Colorize;
        eprintln!("{} {}", "✔".green().bold(), $MSG);
    }};
}

/// Prints a warning message to stderr
#[macro_export]
macro_rules! warn {
    ($MSG:expr) => {{
        use colored::Colorize;
        eprintln!("{} {}", "!".yellow().bold(), $MSG.yellow());
    }};
}

/// Prints an error message to stderr
#[macro_export]
macro_rules! error {
    ($MSG:expr) => {{
        use colored::Colorize;
        eprintln!("{} {}", "✗".red().bold(), $MSG.red());
    }};
}
