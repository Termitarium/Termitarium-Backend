use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::io::Error;

const CONFIG_FILE_NAME: &str = ".termitarium";

fn main() {
    let mut config_file_contents = read_from_config_file().unwrap();
    config_file_contents.push('a');
    println!("{}", config_file_contents);
    write_into_config_file_from(config_file_contents.as_str());
}

fn read_from_config_file() -> Result <String, Error> {
    let config_file = File::open(CONFIG_FILE_NAME)?;

    let mut buf_reader = BufReader::new(config_file);
    let mut config_file_contents = String::new();
    buf_reader.read_to_string(&mut config_file_contents).map(move |_bytes| {
        config_file_contents
    })
}

fn write_into_config_file_from(string: &str) -> Result <usize, Error> {
    let config_file = File::create(CONFIG_FILE_NAME)?;

    let mut buf_writer = BufWriter::new(config_file);
    buf_writer.write(string.as_bytes())
}

// Write Model to file
// Read Model from file
// Alter model with an operation