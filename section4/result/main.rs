fn do_something_that_might_fail(i: i64) -> Result<f32, String> {
  if i == 42 {
    Ok(13.0)
  } else {
    Err(String::from("wrong number"))
  }
}

fn main() {
  let result = do_something_that_might_fail(42);

  match result {
    Ok(v) => println!("discovered: {}", v),
    Err(e) => println!("error: {}", e)
  }
}