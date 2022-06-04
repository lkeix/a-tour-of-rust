fn do_something_that_might_fail(i: i64) -> Result<f32, String> {
  if i == 42 {
    Ok(13.0)
  } else {
    Err(String::from("wrong number"))
  }
}

fn main() -> Result<(), String> {
  let v = do_something_that_might_fail(42)?;
  /*
  do_something_that_might_fail(42)?; same
  match do_something_that_might_fail(42) {
    Ok(v) => v,
    Err(e) => return Err(e)
  }
  */

  println!("{}", v);
  Ok(())
}