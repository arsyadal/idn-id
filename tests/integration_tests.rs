use idn_id::prelude::*;

#[test]
fn test_nik_male_valid() {
    let nik = "3204110609970001";
    let res = parse_nik(nik);
    assert!(res.is_ok());
    let info = res.unwrap();
    assert_eq!(info.province_code, "32");
    assert_eq!(info.province_name, "Jawa Barat");
    assert_eq!(info.gender, Gender::Male);
    assert_eq!(info.birth_day, 6);
    assert_eq!(info.birth_month, 9);
    assert_eq!(info.birth_year, 1997);
    assert_eq!(info.birth_date_iso(), "1997-09-06");
    assert_eq!(info.sequence_number, 1);
}

#[test]
fn test_nik_female_valid() {
    let nik = "3171015508920002";
    let res = parse_nik(nik);
    assert!(res.is_ok());
    let info = res.unwrap();
    assert_eq!(info.province_code, "31");
    assert_eq!(info.province_name, "DKI Jakarta");
    assert_eq!(info.gender, Gender::Female);
    assert_eq!(info.birth_day, 15);
    assert_eq!(info.birth_month, 8);
    assert_eq!(info.birth_year, 1992);
}

#[test]
fn test_nik_invalid_province() {
    let nik = "9904110609970001";
    let res = parse_nik(nik);
    assert!(matches!(res, Err(IdnError::InvalidProvinceCode(_))));
}

#[test]
fn test_nik_invalid_length() {
    let nik = "32041106099700";
    let res = parse_nik(nik);
    assert!(matches!(res, Err(IdnError::InvalidLength { .. })));
}

#[test]
fn test_nik_invalid_date() {
    let nik = "3204113209970001";
    let res = parse_nik(nik);
    assert!(matches!(res, Err(IdnError::InvalidDate { .. })));
}

#[test]
fn test_npwp_15_valid() {
    let npwp = "09.254.294.3-407.000";
    let res = validate_npwp(npwp);
    assert!(res.is_ok());
    let info = res.unwrap();
    assert_eq!(info.is_16_digit, false);
    assert_eq!(info.formatted, "09.254.294.3-407.000");
    assert_eq!(info.kpp_code.as_deref(), Some("407"));
    assert_eq!(info.branch_code.as_deref(), Some("000"));
}

#[test]
fn test_npwp_15_invalid_checksum() {
    let npwp = "09.254.294.9-407.000";
    let res = validate_npwp(npwp);
    assert!(matches!(res, Err(IdnError::InvalidChecksum)));
}

#[test]
fn test_npwp_16_corporate_valid() {
    let npwp16 = "0092542943407000";
    let res = validate_npwp(npwp16);
    assert!(res.is_ok());
    let info = res.unwrap();
    assert_eq!(info.is_16_digit, true);
    assert_eq!(info.formatted, "009.254.294.3-407.000");
}

#[test]
fn test_phone_normalization_and_operator() {
    let p1 = "+62 812-3456-7890";
    let res1 = validate_phone(p1).unwrap();
    assert_eq!(res1.operator, Operator::Telkomsel);
    assert_eq!(res1.national, "081234567890");
    assert_eq!(res1.e164, "+6281234567890");

    let p2 = "0857-1122-3344";
    let res2 = validate_phone(p2).unwrap();
    assert_eq!(res2.operator, Operator::Indosat);

    let p3 = "628181234567";
    let res3 = validate_phone(p3).unwrap();
    assert_eq!(res3.operator, Operator::XLAxiata);
    assert_eq!(res3.national, "08181234567");
}

#[test]
fn test_plate_validation() {
    let plate1 = "B 1234 ABC";
    let res1 = validate_plate(plate1).unwrap();
    assert_eq!(res1.area_code, "B");
    assert_eq!(res1.number, 1234);
    assert_eq!(res1.suffix, "ABC");
    assert!(res1.region_name.contains("DKI Jakarta"));

    let plate2 = "D 999 XY";
    let res2 = validate_plate(plate2).unwrap();
    assert_eq!(res2.area_code, "D");
    assert!(res2.region_name.contains("Bandung"));
}
