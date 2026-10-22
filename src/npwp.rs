use crate::error::IdnError;
use crate::nik;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TaxpayerType {
    Corporate,
    JointOperation,
    StateOwned,
    Individual,
    Other,
}

impl std::fmt::Display for TaxpayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaxpayerType::Corporate => write!(f, "Wajib Pajak Badan"),
            TaxpayerType::JointOperation => write!(f, "Joint Operation / Pengusaha Tertentu"),
            TaxpayerType::StateOwned => write!(f, "BUMN / BUMD"),
            TaxpayerType::Individual => write!(f, "Wajib Pajak Orang Pribadi"),
            TaxpayerType::Other => write!(f, "Lainnya"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NpwpInfo {
    pub raw: String,
    pub formatted: String,
    pub is_16_digit: bool,
    pub taxpayer_type: TaxpayerType,
    pub kpp_code: Option<String>,
    pub branch_code: Option<String>,
}

pub fn verify_luhn(digits: &str) -> bool {
    let mut sum = 0;
    for (i, c) in digits.chars().rev().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            if i % 2 == 1 {
                let prod = digit * 2;
                sum += if prod <= 9 { prod } else { prod - 9 };
            } else {
                sum += digit;
            }
        } else {
            return false;
        }
    }
    sum % 10 == 0
}

pub fn format_15(digits: &str) -> Result<String, IdnError> {
    if digits.len() != 15 || !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err(IdnError::InvalidLength { expected: 15, actual: digits.len() });
    }

    Ok(format!(
        "{}.{}.{}.{}-{}.{}",
        &digits[0..2],
        &digits[2..5],
        &digits[5..8],
        &digits[8..9],
        &digits[9..12],
        &digits[12..15]
    ))
}

pub fn validate(npwp: &str) -> Result<NpwpInfo, IdnError> {
    let clean: String = npwp.chars().filter(|c| !c.is_whitespace() && *c != '-' && *c != '.').collect();

    if clean.is_empty() {
        return Err(IdnError::EmptyInput);
    }

    if !clean.chars().all(|c| c.is_ascii_digit()) {
        return Err(IdnError::InvalidCharacters("NPWP must contain only digits (0-9)"));
    }

    match clean.len() {
        15 => validate_15(&clean),
        16 => validate_16(&clean),
        actual => Err(IdnError::InvalidLength { expected: 15, actual }),
    }
}

fn validate_15(clean: &str) -> Result<NpwpInfo, IdnError> {
    if !verify_luhn(&clean[0..9]) {
        return Err(IdnError::InvalidChecksum);
    }

    let type_prefix = &clean[0..2];
    let taxpayer_type = match type_prefix {
        "01" | "02" | "03" => TaxpayerType::Corporate,
        "04" | "05" => TaxpayerType::JointOperation,
        "06" => TaxpayerType::StateOwned,
        "07" | "08" | "09" => TaxpayerType::Individual,
        _ => TaxpayerType::Other,
    };

    let formatted = format_15(clean)?;
    let kpp_code = Some(clean[9..12].to_string());
    let branch_code = Some(clean[12..15].to_string());

    Ok(NpwpInfo {
        raw: clean.to_string(),
        formatted,
        is_16_digit: false,
        taxpayer_type,
        kpp_code,
        branch_code,
    })
}

fn validate_16(clean: &str) -> Result<NpwpInfo, IdnError> {
    if clean.starts_with('0') {
        let base_15 = &clean[1..];
        if !verify_luhn(&base_15[0..9]) {
            return Err(IdnError::InvalidChecksum);
        }

        let type_prefix = &base_15[0..2];
        let taxpayer_type = match type_prefix {
            "01" | "02" | "03" => TaxpayerType::Corporate,
            "04" | "05" => TaxpayerType::JointOperation,
            "06" => TaxpayerType::StateOwned,
            "07" | "08" | "09" => TaxpayerType::Individual,
            _ => TaxpayerType::Other,
        };

        let formatted = format!("0{}", format_15(base_15)?);
        let kpp_code = Some(base_15[9..12].to_string());
        let branch_code = Some(base_15[12..15].to_string());

        Ok(NpwpInfo {
            raw: clean.to_string(),
            formatted,
            is_16_digit: true,
            taxpayer_type,
            kpp_code,
            branch_code,
        })
    } else {
        nik::validate(clean)?;
        let formatted = format!("{}.{}.{}.{}", &clean[0..4], &clean[4..8], &clean[8..12], &clean[12..16]);
        Ok(NpwpInfo {
            raw: clean.to_string(),
            formatted,
            is_16_digit: true,
            taxpayer_type: TaxpayerType::Individual,
            kpp_code: None,
            branch_code: None,
        })
    }
}
