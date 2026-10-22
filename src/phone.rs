use crate::error::IdnError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Operator {
    Telkomsel,
    Indosat,
    XLAxiata,
    Axis,
    Smartfren,
    Tri,
    Unknown,
}

impl Operator {
    pub fn name(&self) -> &'static str {
        match self {
            Operator::Telkomsel => "Telkomsel (Halo / SimPATI / By.U)",
            Operator::Indosat => "Indosat Ooredoo Hutchison (IM3)",
            Operator::XLAxiata => "XL Axiata",
            Operator::Axis => "Axis",
            Operator::Smartfren => "Smartfren",
            Operator::Tri => "Tri (3)",
            Operator::Unknown => "Unknown / Virtual Operator",
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhoneInfo {
    pub raw: String,
    pub national: String,
    pub e164: String,
    pub operator: Operator,
    pub prefix: String,
}

pub fn detect_operator(prefix: &str) -> Operator {
    match prefix {
        "0811" | "0812" | "0813" | "0821" | "0822" | "0823" | "0851" | "0852" | "0853" => {
            Operator::Telkomsel
        }
        "0814" | "0815" | "0816" | "0855" | "0856" | "0857" | "0858" => Operator::Indosat,
        "0817" | "0818" | "0819" | "0859" | "0877" | "0878" => Operator::XLAxiata,
        "0831" | "0832" | "0833" | "0838" => Operator::Axis,
        "0881" | "0882" | "0883" | "0884" | "0885" | "0886" | "0887" | "0888" | "0889" => {
            Operator::Smartfren
        }
        "0895" | "0896" | "0897" | "0898" | "0899" => Operator::Tri,
        _ => Operator::Unknown,
    }
}

pub fn normalize(phone: &str) -> Result<(String, String), IdnError> {
    let clean: String = phone
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect();

    if clean.is_empty() {
        return Err(IdnError::EmptyInput);
    }

    let digits = if let Some(stripped) = clean.strip_prefix('+') {
        stripped
    } else {
        &clean
    };

    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err(IdnError::InvalidCharacters("Phone number contains non-numeric characters"));
    }

    let national = if digits.starts_with("628") {
        format!("08{}", &digits[3..])
    } else if digits.starts_with("08") {
        digits.to_string()
    } else if digits.starts_with("8") {
        format!("0{}", digits)
    } else {
        return Err(IdnError::UnknownOperatorPrefix(digits.chars().take(4).collect()));
    };

    if national.len() < 10 || national.len() > 14 {
        return Err(IdnError::InvalidLength {
            expected: 12,
            actual: national.len(),
        });
    }

    let e164 = format!("+62{}", &national[1..]);
    Ok((national, e164))
}

pub fn validate(phone: &str) -> Result<PhoneInfo, IdnError> {
    let (national, e164) = normalize(phone)?;
    let prefix = national.chars().take(4).collect::<String>();
    let operator = detect_operator(&prefix);

    if operator == Operator::Unknown {
        return Err(IdnError::UnknownOperatorPrefix(prefix));
    }

    Ok(PhoneInfo {
        raw: phone.to_string(),
        national,
        e164,
        operator,
        prefix,
    })
}
