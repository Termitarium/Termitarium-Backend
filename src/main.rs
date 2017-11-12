extern crate serde_yaml;

extern crate termitarium_lib;
use termitarium_lib::model::*;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::io::Error;


const MODEL_FILE_NAME: &str = ".model";

fn main() {
    let mut config_file_contents = read_from_config_file().unwrap();
    println!("{}", config_file_contents);


//    let model: Model = Model {entities: vec![
//        Entity{id:"1".to_string(), name:"Task".to_string()},
//        Entity{id:"2".to_string(), name:"User Story".to_string()},
//    ]};
    let model: Model = serde_yaml::from_str(&mut config_file_contents).unwrap();;
    write_into_config_file_from(&model);
}

fn read_from_config_file() -> Result <String, Error> {
    let config_file = File::open(MODEL_FILE_NAME)?;

    let mut buf_reader = BufReader::new(config_file);
    let mut config_file_contents = String::new();
    buf_reader.read_to_string(&mut config_file_contents).map(move |_bytes| {
        config_file_contents
    })
}

fn write_into_config_file_from(model: &Model) -> Result <usize, Error> {
    let serialization: String = serde_yaml::to_string(model).unwrap();

    let config_file = File::create(MODEL_FILE_NAME)?;

    let mut buf_writer = BufWriter::new(config_file);
    buf_writer.write(serialization.as_bytes())
}

// Write Model to file
// Read Model from file
// Alter model with an operation