
#[macro_export]
macro_rules! print_error {
    ($($msg:tt)*) => {{
        use colored::*;
        let format = format!($($msg)*);
        eprintln!("{}{}{}: {}", "*".yellow(), "Error".red(), "*".yellow(), format);
    }};
}




