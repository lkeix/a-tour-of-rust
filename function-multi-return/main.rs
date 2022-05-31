fn swap(i: i64, j: i64) -> (i64, i64) {
  return (j, i);
}

fn main() {
  let result = swap(1, 2);

  println!("{}, {}", result.0, result.1);

  let (a, b) = swap(result.0, result.1);

  println!("{}, {}", a, b);
}