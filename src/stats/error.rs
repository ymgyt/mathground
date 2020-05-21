use std::fmt;

#[derive(Debug, PartialEq)]
pub enum StatsError {
    DifferentDataSize,
    Unknown(String),
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use StatsError::*;
        match self {
            DifferentDataSize => write!(f, "different data size"),
            Unknown(msg) => write!(f, "unknown error: {}", msg),
        }
    }
}

impl std::error::Error for StatsError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn impl_std_error() {
        require_std_error(Box::new(StatsError::DifferentDataSize));
    }

    fn require_std_error(_: Box<dyn std::error::Error>) {}
}
