
fn main() {
  #[derive(Debug)]
  struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String
  }

  let octopus = SeaCreature {
    animal_type: String::from("octopus"),
    name: String::from("octo"),
    arms: 8,
    legs: 0,
    weapon: String::from("tentacles")
  };

  println!("{:?}", octopus);
}