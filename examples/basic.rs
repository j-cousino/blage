use toml::{Table, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut table = Table::new();

    table.insert("name".to_string(), Value::String("Jeff Cousino".to_string()));

    println!("{}", table);
    
    Ok(())
}