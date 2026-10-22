use crate::error::IdnError;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlateInfo {
    pub formatted: String,
    pub area_code: String,
    pub region_name: &'static str,
    pub number: u32,
    pub suffix: String,
}

pub fn lookup_area(area: &str) -> Option<&'static str> {
    match area {
        "A" => Some("Banten (Serang, Cilegon, Pandeglang, Lebak, Tangerang)"),
        "B" => Some("DKI Jakarta, Depok, Bekasi, Tangerang"),
        "D" => Some("Bandung Raya (Kota/Kab Bandung, Cimahi, Bandung Barat)"),
        "E" => Some("Cirebon, Indramayu, Majalengka, Kuningan"),
        "F" => Some("Bogor, Cianjur, Sukabumi"),
        "G" => Some("Pekalongan, Tegal, Brebes, Batang, Pemalang"),
        "H" => Some("Semarang, Salatiga, Kendal, Demak"),
        "K" => Some("Pati, Kudus, Jepara, Rembang, Blora, Grobogan"),
        "L" => Some("Surabaya"),
        "M" => Some("Madura (Bangkalan, Sampang, Pamekasan, Sumenep)"),
        "N" => Some("Malang, Probolinggo, Pasuruan, Lumajang, Batu"),
        "P" => Some("Besuki (Jember, Banyuwangi, Bondowoso, Situbondo)"),
        "R" => Some("Banyumas, Cilacap, Purbalingga, Banjarnegara"),
        "S" => Some("Bojonegoro, Tuban, Lamongan, Mojokerto, Jombang"),
        "T" => Some("Purwakarta, Karawang, Subang"),
        "W" => Some("Sidoarjo, Gresik"),
        "Z" => Some("Priangan Timur (Garut, Tasikmalaya, Ciamis, Banjar, Pangandaran)"),
        "AA" => Some("Kedu (Magelang, Kebumen, Purworejo, Wonosobo, Temanggung)"),
        "AB" => Some("DI Yogyakarta"),
        "AD" => Some("Surakarta / Solo Raya (Boyolali, Klaten, Sukoharjo, Wonogiri, Sragen)"),
        "AE" => Some("Madiun, Ngawi, Magetan, Ponorogo, Pacitan"),
        "AG" => Some("Kediri, Blitar, Tulungagung, Nganjuk, Trenggalek"),
        "BA" => Some("Sumatera Barat"),
        "BB" => Some("Sumatera Utara Bagian Barat (Tapanuli, Sibolga, Nias)"),
        "BD" => Some("Bengkulu"),
        "BE" => Some("Lampung"),
        "BG" => Some("Sumatera Selatan"),
        "BH" => Some("Jambi"),
        "BK" => Some("Sumatera Utara Bagian Timur (Medan, Deli Serdang)"),
        "BL" => Some("Aceh"),
        "BM" => Some("Riau"),
        "BN" => Some("Kepulauan Bangka Belitung"),
        "BP" => Some("Kepulauan Riau (Batam, Tanjungpinang)"),
        "DA" => Some("Kalimantan Selatan"),
        "DB" => Some("Sulawesi Utara Daratan (Manado, Bitung, Tomohon, Minahasa)"),
        "DC" => Some("Sulawesi Barat"),
        "DD" => Some("Sulawesi Selatan Bagian Selatan (Makassar, Gowa, Maros)"),
        "DE" => Some("Maluku"),
        "DG" => Some("Maluku Utara"),
        "DH" => Some("Nusa Tenggara Timur (Timor, Rote, Sabu)"),
        "DK" => Some("Bali"),
        "DL" => Some("Sulawesi Utara Kepulauan (Sangihe, Talaud, Sitaro)"),
        "DM" => Some("Gorontalo"),
        "DN" => Some("Sulawesi Tengah"),
        "DP" => Some("Sulawesi Selatan Bagian Utara (Parepare, Palopo, Luwu)"),
        "DR" => Some("Nusa Tenggara Barat (Lombok)"),
        "DT" => Some("Sulawesi Tenggara"),
        "DW" => Some("Sulawesi Selatan Bagian Tengah (Bone, Soppeng, Wajo)"),
        "EA" => Some("Nusa Tenggara Barat (Sumbawa, Bima, Dompu)"),
        "EB" => Some("Nusa Tenggara Timur (Flores, Lembata, Alor)"),
        "ED" => Some("Nusa Tenggara Timur (Sumba)"),
        "KB" => Some("Kalimantan Barat"),
        "KH" => Some("Kalimantan Tengah"),
        "KT" => Some("Kalimantan Timur"),
        "KU" => Some("Kalimantan Utara"),
        "PA" => Some("Papua"),
        "PB" => Some("Papua Barat"),
        _ => None,
    }
}

pub fn validate(plate: &str) -> Result<PlateInfo, IdnError> {
    let clean: String = plate.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_uppercase()).collect();
    if clean.is_empty() {
        return Err(IdnError::EmptyInput);
    }

    let mut area = String::new();
    let mut number_str = String::new();
    let mut suffix = String::new();
    let mut state = 0;

    for c in clean.chars() {
        match state {
            0 => {
                if c.is_ascii_alphabetic() {
                    area.push(c);
                    if area.len() > 2 {
                        return Err(IdnError::InvalidPlateArea(area));
                    }
                } else if c.is_ascii_digit() {
                    state = 1;
                    number_str.push(c);
                } else {
                    return Err(IdnError::InvalidCharacters("Unexpected character in plate"));
                }
            }
            1 => {
                if c.is_ascii_digit() {
                    number_str.push(c);
                    if number_str.len() > 4 {
                        return Err(IdnError::MalformedFormat("Vehicle number portion must be 1 to 4 digits"));
                    }
                } else if c.is_ascii_alphabetic() {
                    state = 2;
                    suffix.push(c);
                } else {
                    return Err(IdnError::InvalidCharacters("Unexpected character in plate"));
                }
            }
            2 => {
                if c.is_ascii_alphabetic() {
                    suffix.push(c);
                    if suffix.len() > 3 {
                        return Err(IdnError::MalformedFormat("Vehicle suffix portion must be 1 to 3 letters"));
                    }
                } else {
                    return Err(IdnError::MalformedFormat("Vehicle suffix must contain only letters"));
                }
            }
            _ => unreachable!(),
        }
    }

    if area.is_empty() || number_str.is_empty() {
        return Err(IdnError::MalformedFormat("Plate must include area code and registration number"));
    }

    let region_name = match lookup_area(&area) {
        Some(name) => name,
        None => return Err(IdnError::InvalidPlateArea(area)),
    };

    let number: u32 = number_str.parse().map_err(|_| IdnError::MalformedFormat("Failed to parse plate number digits"))?;
    if number == 0 {
        return Err(IdnError::MalformedFormat("Plate number cannot be zero"));
    }

    let formatted = if suffix.is_empty() {
        format!("{} {}", area, number)
    } else {
        format!("{} {} {}", area, number, suffix)
    };

    Ok(PlateInfo {
        formatted,
        area_code: area,
        region_name,
        number,
        suffix,
    })
}
