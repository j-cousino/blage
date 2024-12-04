use std::env;
use std::path::PathBuf;
use std::{fs::read_to_string, io::Write};
use toml::Table;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert!( args.len() == 2, "Please specify a filename to translate" );

    let mut path: PathBuf = args[1].clone().into();

    // read the contacts into a string
    let csv = read_to_string( &path )?;

    // if we make it here the file exists and we were able to read it.
    let (name, _ext) = args[1].split_once('.').unwrap();

    // create an iterator over each line 
    let mut lines = csv.lines();

    // get the first line that defines the fields
    let heading = lines.next().expect("No first line in the file.");

    let field_names: Vec<&str> = heading.split(',').collect();

    let mut records: Vec<toml::Value> = vec![];
    
    for line in lines {
        records.push(toml::Value::Table(create_toml(&field_names, line)));
    }

    let mut toml = Table::new();

    toml.insert(name.to_string(), toml::Value::Array(records));
    
    path.set_extension("toml");

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;

    file.write_all(toml.to_string().as_bytes())?;

    anyhow::Ok(())
}

fn create_toml(field_names: &Vec<&str>, line: &str) -> Table {
    let field_values: Vec<&str> = line.split(',').collect();
    
    let count = || -> usize {
        if field_names.len() <= field_values.len() {
            field_names.len()-1
        } else {
            field_values.len()-1
        }
    };
    
    let mut contact = Table::new();

    for i in 0..count() {
        contact.insert(field_names[i].to_string(), toml::Value::String(field_values[i].to_string()));
    }

    contact
}