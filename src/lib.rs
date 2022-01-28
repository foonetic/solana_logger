//! `solana_logger` defines a collection of logging macros that are enabled and
//! disabled at compile time. 
//! 
//! The user is responsible for calling `set_log_level` for each module in which
//! logging is enabled. `debug`, `info`, `warn`, and `error` macros will then be
//! enabled if and only if the corresponding level is at least the set level.
//! 
//! For example:
//! 
//! ```rust
//! 
//! // Only Info, Warn, and Error will be printed.
//! set_log_level!(Info);
//! 
//! debug!("This will be skipped");
//! info!("This will be printed"); 
//! ```
//! 
//! The logging macros support the same string formatting as `solana_program::msg`.

/// Represents a logging level. Levels are ordered from least to most important
/// as `Debug`, `Info`, `Warn`, and `Error`.
#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,

    /// Set the `Disabled` logging level to disable all logging.
    Disabled,
}

/// Conditionally logs a message. Users should prefer one of the predefined
/// message macros `debug`, `info`, `warn`, or `error`.
#[macro_export]
macro_rules! log {
    (prefix $label:expr, $fmt:expr) => {
        concat!("[", file!(), ":", line!(), " ", $label, "] ", $fmt)
    };
    ($level: expr, $label: expr, $fmt:expr, $($opt:expr),*) => {
        if self::SolanaLoggerLogLevel::level() <= $level {
            solana_program::msg!($crate::log!(prefix $label, $fmt), $($opt),*);
        }
    };
    ($level: expr, $label: expr, $opt:expr) => {
        if self::SolanaLoggerLogLevel::level() <= $level {
            solana_program::msg!($crate::log!(prefix $label, "{}"), $opt);
        } 
    };
}

/// Emits a message if the logging level is set to `Debug` or below.
#[macro_export]
macro_rules! debug {
    ($($opt:expr),*) => {
        $crate::log!($crate::Level::Debug, "DEBUG", $($opt),*);
    };
}

/// Emits a message if the logging level is set to `Info` or below.
#[macro_export]
macro_rules! info {
    ($($opt:expr),*) => {
        $crate::log!($crate::Level::Info, "INFO", $($opt),*);
    };
}

/// Emits a message if the logging level is set to `Warn` or below.
#[macro_export]
macro_rules! warn {
    ($($opt:expr),*) => {
        $crate::log!($crate::Level::Warn, "WARN", $($opt),*);
    };
}

/// Emits a message if the logging level is set to `Error` or below.
#[macro_export]
macro_rules! error {
    ($($opt:expr),*) => {
        $crate::log!($crate::Level::Error, "ERROR", $($opt),*);
    };
}

/// Sets the log level for the current module.
#[macro_export]
macro_rules! set_log_level {
    ($level: expr) => {
        struct SolanaLoggerLogLevel;

        impl SolanaLoggerLogLevel {
            pub fn level() -> $crate::Level {
                $level
            }
        }
    }
}

#[cfg(test)]
mod tests {
    set_log_level!(crate::Level::Warn);

    #[test]
    fn ignore_info() {
        info!("should not display");
    }

    #[test]
    fn include_self() {
        warn!("should display");
    }

    #[test]
    fn include_above() {
        error!("should display");
    }

    #[test]
    fn apply_formatting() {
        error!("hello {}", "world");
    }
}
