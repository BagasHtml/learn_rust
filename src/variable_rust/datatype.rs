pub fn datatype() {
  const ANGKA: i16 = 300;
  const NAME: &str = "Bagas Tresna Nanda MS";
  const BILANGAN_BULAT: f32 = 3.14;
  const STATUS: bool = true;
  let numbers = [3, 2, 1, 4];

  println!("200 + 100 = {}", ANGKA);
  println!("Halo! nama saya {}", NAME);
  println!("Phi berjumlah {}", BILANGAN_BULAT);
  println!("Apakah hari ini tidak akan turun hujan? {}", STATUS);
  println!("bilangan index ke-2 pada array numbers adalah: {}", numbers[2]);
}