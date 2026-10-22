use crate::error::IdnError;

/// Gender extracted from NIK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Gender {
    Male,
    Female,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Laki-laki"),
            Gender::Female => write!(f, "Perempuan"),
        }
    }
}

/// Parsed metadata from an Indonesian NIK (Nomor Induk Kependudukan).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NikInfo {
    pub nik: String,
    pub province_code: &'static str,
    pub province_name: &'static str,
    pub regency_code: String,
    pub district_code: String,
    pub birth_day: u32,
    pub birth_month: u32,
    pub birth_year: i32,
    pub gender: Gender,
    pub sequence_number: u32,
}

impl NikInfo {
    pub fn birth_date_iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.birth_year, self.birth_month, self.birth_day)
    }

    pub fn age(&self, current_year: i32) -> u32 {
        if current_year >= self.birth_year {
            (current_year - self.birth_year) as u32
        } else {
            0
        }
    }
}

pub fn lookup_province(code: &str) -> Option<(&'static str, &'static str)> {
    match code {
        "11" => Some(("11", "Aceh")),
        "12" => Some(("12", "Sumatera Utara")),
        "13" => Some(("13", "Sumatera Barat")),
        "14" => Some(("14", "Riau")),
        "15" => Some(("15", "Jambi")),
        "16" => Some(("16", "Sumatera Selatan")),
        "17" => Some(("17", "Bengkulu")),
        "18" => Some(("18", "Lampung")),
        "19" => Some(("19", "Kepulauan Bangka Belitung")),
        "21" => Some(("21", "Kepulauan Riau")),
        "31" => Some(("31", "DKI Jakarta")),
        "32" => Some(("32", "Jawa Barat")),
        "33" => Some(("33", "Jawa Tengah")),
        "34" => Some(("34", "DI Yogyakarta")),
        "35" => Some(("35", "Jawa Timur")),
        "36" => Some(("36", "Banten")),
        "51" => Some(("51", "Bali")),
        "52" => Some(("52", "Nusa Tenggara Barat")),
        "53" => Some(("53", "Nusa Tenggara Timur")),
        "61" => Some(("61", "Kalimantan Barat")),
        "62" => Some(("62", "Kalimantan Tengah")),
        "63" => Some(("63", "Kalimantan Selatan")),
        "64" => Some(("64", "Kalimantan Timur")),
        "65" => Some(("65", "Kalimantan Utara")),
        "71" => Some(("71", "Sulawesi Utara")),
        "72" => Some(("72", "Sulawesi Tengah")),
        "73" => Some(("73", "Sulawesi Selatan")),
        "74" => Some(("74", "Sulawesi Tenggara")),
        "75" => Some(("75", "Gorontalo")),
        "76" => Some(("76", "Sulawesi Barat")),
        "81" => Some(("81", "Maluku")),
        "82" => Some(("82", "Maluku Utara")),
        "91" => Some(("91", "Papua Barat")),
        "92" => Some(("92", "Papua Barat Daya")),
        "93" => Some(("93", "Papua Selatan")),
        "94" => Some(("94", "Papua Tengah")),
        "95" => Some(("95", "Papua Pegunungan")),
        "96" => Some(("96", "Papua")),
        _ => None,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(month: u32, year: i32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0,
    }
}

pub fn validate(nik: &str) -> Result<(), IdnError> {
    parse(nik).map(|_| ())
}

pub fn parse(nik: &str) -> Result<NikInfo, IdnError> {
    parse_with_year(nik, 2026)
}

pub fn parse_with_year(nik: &str, current_year: i32) -> Result<NikInfo, IdnError> {
    let clean: String = nik.chars().filter(|c| !c.is_whitespace() && *c != '-' && *c != '.').collect();

    if clean.is_empty() {
        return Err(IdnError::EmptyInput);
    }

    if clean.len() != 16 {
        return Err(IdnError::InvalidLength { expected: 16, actual: clean.len() });
    }

    if !clean.chars().all(|c| c.is_ascii_digit()) {
        return Err(IdnError::InvalidCharacters("NIK must contain only numeric digits (0-9)"));
    }

    let prov_code = &clean[0..2];
    let (p_code, p_name) = match lookup_province(prov_code) {
        Some(entry) => entry,
        None => return Err(IdnError::InvalidProvinceCode(prov_code.to_string())),
    };

    let regency_code = clean[2..4].to_string();
    let district_code = clean[4..6].to_string();

    let raw_day: u32 = clean[6..8].parse().map_err(|_| IdnError::MalformedFormat("Cannot parse day digits"))?;
    let birth_month: u32 = clean[8..10].parse().map_err(|_| IdnError::MalformedFormat("Cannot parse month digits"))?;
    let year_2digit: i32 = clean[10..12].parse().map_err(|_| IdnError::MalformedFormat("Cannot parse year digits"))?;

    let (birth_day, gender) = if raw_day > 40 {
        (raw_day - 40, Gender::Female)
    } else {
        (raw_day, Gender::Male)
    };

    if birth_month < 1 || birth_month > 12 {
        return Err(IdnError::InvalidDate { day: birth_day, month: birth_month, year: year_2digit });
    }

    let current_yy = current_year % 100;
    let birth_year = if year_2digit <= current_yy {
        (current_year / 100) * 100 + year_2digit
    } else {
        ((current_year / 100) - 1) * 100 + year_2digit
    };

    let max_days = days_in_month(birth_month, birth_year);
    if birth_day < 1 || birth_day > max_days {
        return Err(IdnError::InvalidDate { day: birth_day, month: birth_month, year: birth_year });
    }

    let sequence_number: u32 = clean[12..16].parse().map_err(|_| IdnError::MalformedFormat("Cannot parse sequence digits"))?;
    if sequence_number == 0 {
        return Err(IdnError::MalformedFormat("Sequence number cannot be 0000"));
    }

    Ok(NikInfo {
        nik: clean,
        province_code: p_code,
        province_name: p_name,
        regency_code,
        district_code,
        birth_day,
        birth_month,
        birth_year,
        gender,
        sequence_number,
    })
}
