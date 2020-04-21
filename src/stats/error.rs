use std::fmt;

#[derive(Debug)]
pub enum StatsError {
    Unknown(String),
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatsError::Unknown(msg) => write!(f, "unknown error: {}", msg),
        }
    }
}

impl std::error::Error for StatsError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn impl_std_error() {
        require_std_error(Box::new(StatsError::Unknown));
    }

    fn require_std_error(_: Box<dyn std::error::Error>) {}
}
