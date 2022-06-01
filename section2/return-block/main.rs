fn example() -> i32 {
  let x = 42;

  let v = if x < 42 { -1 } else { 1 };

  println!("v is {}", v);

  let food = "humburger";
  let result = match food {
    "hotdog" => "food is hotdog",
    _ => "food is not hotdog",
  };
  println!("food category: {}", result);

  let v = {
    let a = 1;
    let b = 2;
    a + b
  };

  println!("v is {}", v);

  // rust will infer the type of the return value
  v + 4
}

fn main() {
  println!("example returned: {}", example())
}