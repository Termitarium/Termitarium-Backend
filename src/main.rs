extern crate serde_yaml;

extern crate termitarium_lib;
use termitarium_lib::model::*;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::io::Error;



extern crate tui;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, border, Widget};
use tui::layout::{Group, Rect, Direction, Size};
use tui::style::Style;
use tui::style::Color;

// DElete me
use std::{thread, time};

const MODEL_FILE_NAME: &str = ".model";

fn main() {
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();


    let size = terminal.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(size.height-3), Size::Fixed(1)])
        .render(&mut terminal, &size, |mut terminal, chunks| {
            Block::default()
                .title("List")
                .borders(border::ALL)
                .style(Style::default().bg(Color::Black))
                .render(terminal, &chunks[0]);
            Block::default()
                .title("Command")
                .borders(border::ALL)
                .style(Style::default().bg(Color::Black))
                .render(terminal, &chunks[1]);
        });


    terminal.draw().unwrap();



    thread::sleep(time::Duration::from_secs(20));



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
