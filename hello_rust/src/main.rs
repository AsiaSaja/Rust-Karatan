fn main() {
    // 1. immutable
    let nama = "Budi";
    println!("Halo, {}", nama);
    
    // 2. mutable
    let mut umur = 20;
    umur = 21;
    println!("Umur: {}", umur);
    
    // 3. shadowing dengan tipe berubah
    let nilai = "100";
    println!("Nilai sebagai string: {}", nilai);
    let nilai: i32 = nilai.parse().unwrap();
    println!("Nilai sebagai angka + 1: {}", nilai + 1);
}