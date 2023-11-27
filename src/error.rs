#[derive(Debug, Clone)]
pub struct ShotError {
    message: String,
}

impl std::fmt::Display for ShotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for ShotError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<ShotError> for std::io::Error {
    fn from(value: ShotError) -> Self {
        Self::new(std::io::ErrorKind::Other, value.message)
    }
}
