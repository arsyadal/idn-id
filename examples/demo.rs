use idn_id::prelude::*;

fn main() {
    println!("=== DEMO PUSTAKA idn-id (Validasi Data Identitas Indonesia) ===
");

    // 1. NIK - Laki-laki
    let nik_pria = "3204110609970001";
    println!("--- 1. Validasi NIK Pria ({}) ---", nik_pria);
    match parse_nik(nik_pria) {
        Ok(info) => {
            println!("  Status         : Valid");
            println!("  Provinsi       : {} (Kode: {})", info.province_name, info.province_code);
            println!("  Tanggal Lahir  : {}", info.birth_date_iso());
            println!("  Jenis Kelamin  : {}", info.gender);
            println!("  Nomor Urut     : {:04}", info.sequence_number);
            println!("  Perkiraan Usia : {} tahun", info.age(2026));
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // 2. NIK - Perempuan (Tanggal +40)
    let nik_wanita = "3171015508920002"; // 55 -> tanggal 15, bulan 08, tahun 1992
    println!("--- 2. Validasi NIK Perempuan ({}) ---", nik_wanita);
    match parse_nik(nik_wanita) {
        Ok(info) => {
            println!("  Status         : Valid");
            println!("  Provinsi       : {}", info.province_name);
            println!("  Tanggal Lahir  : {}", info.birth_date_iso());
            println!("  Jenis Kelamin  : {}", info.gender);
            println!("  Perkiraan Usia : {} tahun", info.age(2026));
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // 3. NPWP 15-Digit (Luhn Validated)
    let npwp_15 = "09.254.294.3-407.000";
    println!("--- 3. Validasi NPWP 15-Digit ({}) ---", npwp_15);
    match validate_npwp(npwp_15) {
        Ok(info) => {
            println!("  Status         : Valid (Luhn Checksum Terverifikasi)");
            println!("  Format Standar : {}", info.formatted);
            println!("  Tipe Wajib Pajak: {}", info.taxpayer_type);
            println!("  Kode KPP       : {:?}", info.kpp_code);
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // 4. Nomor Handphone
    let phone = "+62 812-3456-7890";
    println!("--- 4. Validasi Nomor Telepon ({}) ---", phone);
    match validate_phone(phone) {
        Ok(info) => {
            println!("  Status         : Valid");
            println!("  Format Nasional: {}", info.national);
            println!("  Format E.164   : {}", info.e164);
            println!("  Operator       : {}", info.operator);
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // 5. Plat Nomor Kendaraan
    let plat = "B 1234 ABC";
    println!("--- 5. Validasi Plat Nomor ({}) ---", plat);
    match validate_plate(plat) {
        Ok(info) => {
            println!("  Status         : Valid");
            println!("  Format Standar : {}", info.formatted);
            println!("  Kode Wilayah   : {}", info.area_code);
            println!("  Cakupan Daerah : {}", info.region_name);
        }
        Err(e) => println!("  Error: {}", e),
    }
}
