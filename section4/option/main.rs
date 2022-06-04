struct BagOfHolding<T> {
  item: Option<T>,
}

fn main() {
  let i32_bag = BagOfHolding::<i32> { item: None };

  if i32_bag.item.is_none() {
    println!("i32_bag is empty");
  } else {
    println!("i32_bag is not empty");
  }

  let i32_bag = BagOfHolding::<i32> { item: Some(42) };

  if i32_bag.item.is_some() {
    println!("i32_bag is not empty");
  } else {
    println!("i32_bag is empty");
  }

  match i32_bag.item {
    Some(v) => println!("i32_bag has a value: {}", v),
    None => println!("i32_bag is empty")
  }
}