fn main() {
  let x = 12; // デフォルトはi32
  let a = 12u8; // unsigned 8bit
  let b = 4.3; // デフォルトはf64
  let c = 4.3f32; // float 32bit
  let bv = true; // bool
  let sentence = "hello world"; // str

  println!("{} {} {} {} {} {}", x, a, b, c, bv, sentence);
}