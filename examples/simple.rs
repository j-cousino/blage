//! Create an object defined with toml formatted text.
use pam::*;

fn main() -> Result<(), Box<()>> {
    let person = toml::toml! {
        name = { first = "String", last = "String" }
        email = "String"
        adress = ["String", "String", "String"]
    };

    println!("{}", person);

    let props = person.iter();
    let keys = person.keys();
    
    for (key, value) in props {
        match value {
            toml::Value::Array(array) => {
                println!("{}: {:?}", key, array);
            }
            other => {
                println!("{}: {}", key, value);    
            }
        }
    }

    for key in keys {
        println!("{}", key);
    }

    Ok(())
}