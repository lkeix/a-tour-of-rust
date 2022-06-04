fn do_something_that_might_fail(i: i64) -> Result<f32, String> {
  if i == 42 {
    Ok(13.0)
  } else {
    Err(String::from("wrong number"))
  }
}

// main return empty or error string
fn main() -> Result<(), String> {
  let result = do_something_that_might_fail(12);

  match result {
    Ok(v) => println!("discovered: {}", v),
    Err(_e) => {
      // error handling

      return Err(String::from("error: an error occurred!"));
    }
  }

  Ok(())
}