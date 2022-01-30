//! `solana_logger` defines a collection of logging macros that are enabled and
//! disabled at compile time.
//!
//! The user must build with feature `loglevel_{level}` to enable logging at or
//! above that level. For example, `loglevel_info` will enable ingo, warn, and
//! error logs but disable debug logs.
//!
//! Usage example:
//!
//! ```rust
//!
//! use solana_logger::{debug, info};
//!
//! debug!("This is a debug message!");
//! info!("This is an info message");
//! ```
//!
//! The logging macros support the same string formatting as `solana_program::msg`.

/// Conditionally logs a message. Users should prefer one of the predefined
/// message macros `debug`, `info`, `warn`, or `error`.
#[macro_export]
macro_rules! log {
    (prefix $label:expr, $fmt:expr) => {
        concat!("[", file!(), ":", line!(), " ", $label, "] ", $fmt)
    };
    ($label: expr, $fmt:expr, $($opt:expr),*) => {
		solana_program::msg!($crate::log!(prefix $label, $fmt), $($opt),*);
    };
    ($label: expr, $opt:expr) => {
		solana_program::msg!($crate::log!(prefix $label, "{}"), $opt);
    };
}

/// Emits a message if the logging level is set to `Debug` or below.
#[macro_export]
macro_rules! debug {
    ($($opt:expr),*) => {
		#[cfg(feature = "loglevel_debug")]
        $crate::log!("DEBUG", $($opt),*);
    };
}

/// Emits a message if the logging level is set to `Info` or below.
#[macro_export]
macro_rules! info {
    ($($opt:expr),*) => {
		#[cfg(feature = "loglevel_info")]
        $crate::log!("INFO", $($opt),*);
    };
}

/// Emits a message if the logging level is set to `Warn` or below.
#[macro_export]
macro_rules! warn {
    ($($opt:expr),*) => {
		#[cfg(feature = "loglevel_warn")]
        $crate::log!("WARN", $($opt),*);
    };
}

/// Emits a message if the logging level is set to `Error` or below.
#[macro_export]
macro_rules! error {
    ($($opt:expr),*) => {
		#[cfg(feature = "loglevel_error")]
        $crate::log!("ERROR", $($opt),*);
    };
}

#[cfg(test)]
mod tests {
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
