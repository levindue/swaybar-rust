use serde::{Deserialize, Serialize};

use std::process::Command;

#[derive(Serialize)]
struct Header {
    version: u8,
    //click_events: bool,
    //cont_signal: u8,
    //stop_signal: u8,
}

impl Header {
    pub fn new() -> Header {
        Header {
            version: 1,
            //click_events: false,
            //cont_signal: 18,
            //stop_signal: 19,
       }
    }
}

#[derive(Debug, Serialize)]
struct Block {
    name: String,
    full_text: String,
//    short_text: String,
//    color: String,
//    background: String,
//    border: String,
//    border_top: u8,
//    border_bottom: u8,
//    border_left: u8,
//    border_right: u8,
//    min_width: u8,
//    align: String,
//    instance: String,
//    urgent: bool,
//    seperator: bool,
//    seperator_block_width: u8,
//    markup: String,
}

impl Block {
    pub fn new(name: &str) -> Self {
        Block {
            name: name.to_string(),
            full_text: "".to_string(),
        }
    }

    pub fn set_full_text(&mut self, full_text: &str) {
        self.full_text = full_text.to_string();
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ClickEvent {
    name: String,
    instance: String,
    x: u32,
    y: u32,
    button: u8,
    event: u8,
    relative_x: u32,
    relative_y: u32,
    width: u32,
    height: u32,
}

fn print_header() {
    let header = Header::new();
    let header_json = serde_json::to_string(&header).unwrap();
    println!("{}", header_json);
}

fn test() -> Block {
    let mut block = Block::new("clock");
    let output = Command::new("date")
        .output()
        .expect("failed to execute process");

    let text = String::from_utf8_lossy(&output.stdout);

    block.set_full_text(&text);
    block
}

fn main() {
    print_header();

    let mut body = Vec::new();

    let clock = test();
    body.push(clock);

    let body_json = serde_json::to_string(&body).unwrap();
    print!("[");
    print!("{}", body_json);
    print!(",");
    println!();

    println!("{}", body_json);
    loop {}
}
