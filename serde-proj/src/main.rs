use serde::{Serialize, Deserialize};
use std::fs; 


#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    address: Address
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Address {
    landmark: String,
    near_city: String,
    state: String,
    district: String,
    country: String,
    pincode: String,
    
}

fn main() {
    let person = Person {
        age: 21,
        name: "Raja Rana".to_string(),
        address: Address { 
            landmark: "Near mamta hotel, Haldapani".to_string(), 
            near_city: "Gopeshwar".to_string(), 
            district: "Chamoli".to_string(),
            state: "Uttarkhand".to_string(),
            country: "India".to_string(), 
            pincode: "246401".to_string(),
         }
    }; 

    let json = serde_json::to_string_pretty(&person).unwrap(); 
    let yaml = serde_yaml::to_string(&person).unwrap(); 
    let toml = toml::to_string(&person).unwrap(); 

    fs::write("person.json", &json).unwrap();
    fs::write("person.yaml", &yaml).unwrap(); 
    fs::write("person.toml", &toml).unwrap(); 


    let json_data = fs::read("person.json").unwrap(); 
    let yaml_data = fs::read("person.yaml").unwrap(); 
    let toml_data = fs::read("person.toml").unwrap();  

    let parse_from_json: Person = serde_json::from_slice(&json_data).unwrap(); 
    let parse_from_yaml : Person = serde_yaml::from_slice(&yaml_data).unwrap(); 
    let parse_from_toml: Person = toml::from_slice(&toml_data).unwrap(); 

    println!("Parsed Json: {:?}", parse_from_json); 
    println!("Parsed Yaml: {:?}", parse_from_yaml); 
    println!("Parsed Toml: {:?}", parse_from_toml); 
}
