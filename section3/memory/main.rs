struct SeaCreature {
  animal_type: String,
  name: String,
  arms: i32,
  legs: i32,
  weapon: String,
}

fn main() {
  let ferris = SeaCreature {
    animal_type: String::from("crab"),
    name: String::from("ferris"),
    arms: 2,
    legs: 4,
    weapon: String::from("claws"),
  };

  let sarah = SeaCreature {
    animal_type: String::from("octopus"),
    name: String::from("Sarah"),
    arms: 8,
    legs: 0,
    weapon: String::from("none"),
  };

  println!("{} is a {}. They have {} arms, {} legs, and {} weapon",
    ferris.name,
    ferris.animal_type,
    ferris.arms,
    ferris.legs,
    ferris.weapon
  );

  println!("{} is a {}. They have {} arms, and {} legs. They have no weapon..",
    sarah.name,
    sarah.animal_type,
    sarah.arms,
    sarah.legs
  );
}