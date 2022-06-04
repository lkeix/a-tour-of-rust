fn do_something_that_might_fail(i: i64) -> Result<f32, String> {
  if i == 42 {
    Ok(13.0)
  } else {
    Err(String::from("wrong number"))
  }
}

fn main() -> Result<(), String> {
  let v = do_something_that_might_fail(42).unwrap();

  println!("{}", v);

  // runtime error
  let v = do_something_that_might_fail(1).unwrap();
  println!("{}", v);

  Ok(())
}