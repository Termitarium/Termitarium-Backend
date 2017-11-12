extern crate termitarium_lib;
use termitarium_lib::model::*;

extern crate tui;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, border, Widget};
use tui::layout::{Group, Direction, Size};
use tui::style::Style;
use tui::style::Color;

// Delete me
use std::{thread, time};

fn main() {
    let backend = TermionBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();


    let size = terminal.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(size.height-3), Size::Fixed(1)])
        .render(&mut terminal, &size, |terminal, chunks| {
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