use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
enum ProgrammerType {
    Brilliant,
    Plodding,
    Clueless
}


#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    height_inches:  u8,
    books: Vec<String>,
    programmer_type: ProgrammerType
}


fn main() {
    let person = Person{
        name: "John".to_string(), 
        height_inches: 73, 
        books: vec!["At the Existentialist Cafe".to_string()], 
        programmer_type: ProgrammerType::Plodding
    };

    // Convert the Person to a JSON string.
    let serialized = serde_json::to_string(&person).unwrap();

    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Person.
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);
}