fn main() {
  let a = 13u8;
  let b = 7u32;

  let c = a as u32 + b; // as で数値型を簡単に変換できる
  println!("{}", c);

  let t = true;
  println!("{}", t as u8); // as で数値型を簡単に変換できる
} 