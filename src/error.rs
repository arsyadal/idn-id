use core::fmt;

/// Errors that can occur during validation or parsing of Indonesian identity data.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IdnError {
    EmptyInput,
    InvalidLength { expected: usize, actual: usize },
    InvalidCharacters(&'static str),
    InvalidDate { day: u32, month: u32, year: i32 },
    InvalidProvinceCode(String),
    InvalidChecksum,
    UnknownOperatorPrefix(String),
    InvalidPlateArea(String),
    MalformedFormat(&'static str),
}

impl fmt::Display for IdnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Input string cannot be empty"),
            Self::InvalidLength { expected, actual } => {
                write!(f, "Invalid length: expected {} digits/chars, got {}", expected, actual)
            }
            Self::InvalidCharacters(msg) => write!(f, "Invalid characters: {}", msg),
            Self::InvalidDate { day, month, year } => {
                write!(f, "Invalid birth date: {:02}/{:02}/{:04}", day, month, year)
            }
            Self::InvalidProvinceCode(code) => {
                write!(f, "Unrecognized Indonesian province code: '{}'", code)
            }
            Self::InvalidChecksum => write!(f, "Checksum verification failed (invalid check digit)"),
            Self::UnknownOperatorPrefix(p) => {
                write!(f, "Unrecognized cellular operator prefix: '{}'", p)
            }
            Self::InvalidPlateArea(area) => {
                write!(f, "Unrecognized vehicle plate area code: '{}'", area)
            }
            Self::MalformedFormat(msg) => write!(f, "Malformed format: {}", msg),
        }
    }
}

impl std::error::Error for IdnError {}
