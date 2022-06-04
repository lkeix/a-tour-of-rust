fn main() {
  let mut i64_vec = Vec::<i64>::new();
  i64_vec.push(42);
  i64_vec.push(13);
  i64_vec.push(3);

  // type inference vector
  let mut float_vec = Vec::new();
  float_vec.push(1.3);
  float_vec.push(2.4);
  float_vec.push(3.5);

  // smart macro!
  let string_vec = vec![String::from("Hello"), String::from("Wrold")];

  for word in string_vec.iter() {
    println!("{}", word);
  }
}