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
}