pub mod error;
pub mod nik;
pub mod npwp;
pub mod phone;
pub mod plate;

pub use error::IdnError;
pub use nik::{Gender, NikInfo};
pub use npwp::{NpwpInfo, TaxpayerType};
pub use phone::{Operator, PhoneInfo};
pub use plate::PlateInfo;

/// Validates whether a string is a valid 16-digit Indonesian NIK.
pub fn validate_nik(nik: &str) -> Result<(), IdnError> {
    nik::validate(nik)
}

/// Parses an Indonesian NIK into structured metadata (birth date, gender, province).
pub fn parse_nik(nik: &str) -> Result<NikInfo, IdnError> {
    nik::parse(nik)
}

/// Parses an Indonesian NIK with a specific reference year.
pub fn parse_nik_with_year(nik: &str, current_year: i32) -> Result<NikInfo, IdnError> {
    nik::parse_with_year(nik, current_year)
}

/// Validates an Indonesian NPWP (15-digit legacy or 16-digit integrated format).
pub fn validate_npwp(npwp: &str) -> Result<NpwpInfo, IdnError> {
    npwp::validate(npwp)
}

/// Formats a 15-digit NPWP into the canonical representation XX.XXX.XXX.X-XXX.XXX.
pub fn format_npwp_15(digits: &str) -> Result<String, IdnError> {
    npwp::format_15(digits)
}

/// Validates and normalizes an Indonesian cellular telephone number.
pub fn validate_phone(phone: &str) -> Result<PhoneInfo, IdnError> {
    phone::validate(phone)
}

/// Validates and parses an Indonesian vehicle registration plate.
pub fn validate_plate(plate: &str) -> Result<PlateInfo, IdnError> {
    plate::validate(plate)
}

/// Prelude module for convenient glob importing.
pub mod prelude {
    pub use crate::{
        parse_nik, parse_nik_with_year, validate_nik, validate_npwp, validate_phone,
        validate_plate, Gender, IdnError, NikInfo, NpwpInfo, Operator, PhoneInfo, PlateInfo,
        TaxpayerType,
    };
}
