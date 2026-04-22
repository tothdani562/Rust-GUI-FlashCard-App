#[macro_export]
macro_rules! validation_error {
    ($($arg:tt)*) => {
        $crate::services::validation::ValidationError::new(format!($($arg)*))
    };
}
