fn main() {
  let x = 19;
  match x {
    0 => {
      println!("found zero");
    }
    1 | 2 => {
      println!("found 1 or 2");
    }
    3..=9 => {
      println!("found a number 3 to 9 inclusively");
    }

    matched_num @ 10..=100 => {
      println!("found {} number between 10 to 100!", matched_num);
    }

    _ => {
      println!("found something else");
    }
  }
}