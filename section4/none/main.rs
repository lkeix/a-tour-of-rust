enum Item {
  Inventory(String),
  None
}

struct BagOfHolding {
  item: Item
}

fn main() {
  let item = BagOfHolding { item: Item::Inventory(String::from("boom!")) };
}