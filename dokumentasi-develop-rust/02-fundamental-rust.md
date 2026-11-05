# Topik 1: Variabel & Mutability

- Rust: immutable by default → `let x = 5;` (tidak bisa diubah)
- Untuk mutable → `let mut x = 5;`
- Shadowing: `let x = 5; let x = "teks";` → variabel baru, tipe boleh berubah
- Perbandingan: Python/JS semua mutable; Rust memaksa pikirkan mutability
- Error yang pernah dilihat: E0384 (assign twice to immutable)
  