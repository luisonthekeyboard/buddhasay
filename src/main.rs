use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;

const WRAP_INDEX: usize = 37;
const BUDDHA: [&str; 7] = [
    " \\",
    "  \\    ___",
    "   \\  (-_-) ",
    "      _) (_ ",
    "     /     \\",
    "   _( \\_ _/ )_",
    "  (_____\\_____)",
];

#[derive(Debug)]
struct Message<'a> {
    lines: Vec<&'a str>,
    max_len: usize,
}

#[derive(Debug, Deserialize)]
struct Buddhasay {
    fortune: Vec<Fortune>,
}

#[derive(Debug, Deserialize)]
struct Fortune {
    sentence: String,
}

fn main() {
    let text = read_fortunes();
    let buddhasay_db: Buddhasay = toml::from_str(text.as_str()).unwrap();

    let index = rand::thread_rng().gen_range(0, buddhasay_db.fortune.len());
    let message = split_sentence(&buddhasay_db.fortune[index].sentence);

    draw_message(&message);
}

fn read_fortunes() -> String {
    let mut input = String::new();

    if let Ok(mut file) = File::open("/usr/share/buddhasay/buddhasay.toml") {
        file.read_to_string(&mut input).unwrap();
    } else if let Ok(mut file) = File::open("resources/buddhasay.toml") {
        file.read_to_string(&mut input).unwrap();
    } else {
        println!("No Buddhasay fortunes to read from...  :-(");
        std::process::exit(1)
    }

    input
}

fn fill_line(beg_ch: char, mid_ch: char, mid_len: usize, end_chr: Option<char>) {
    print!("{}", beg_ch);
    for _ in 0..mid_len {
        print!("{}", mid_ch);
    }
    if end_chr != None {
        print!("{}", end_chr.unwrap());
    }
    println!();
}

fn draw_message(m: &Message) {
    // Header
    fill_line(' ', '_', m.max_len + 2, None);
    fill_line('/', ' ', m.max_len + 2, Some('\\'));

    // Message
    for &item in m.lines.iter() {
        print!("| ");
        print!("{}", item);

        if item.len() != m.max_len {
            for _ in 0..(m.max_len - item.len()) {
                print!(" ");
            }
        }

        println!(" |");
    }

    // Footer
    fill_line('\\', '_', m.max_len + 2, Some('/'));

    // Buddha
    let offset = m.max_len - (m.max_len / 3);
    for &buddha_line in BUDDHA.iter() {
        for _ in 0..offset {
            print!(" ");
        }
        println!("{}", &buddha_line);
    }
}

fn previous_whitespace_index(s: &str, i: usize) -> usize {
    let mut index = i;
    let b = s.as_bytes();

    while b[index] != b' ' {
        index -= 1;
    }

    index
}

fn split_sentence(s: &str) -> Message {
    let mut message = Message {
        lines: Vec::new(),
        max_len: 0,
    };

    if s.len() <= WRAP_INDEX {
        message.lines.push(s.trim());
        message.max_len = s.trim().len();
    } else {
        let mut index = WRAP_INDEX;
        let mut last_end = 0;

        loop {
            let prev_whtspc_idx = if index < s.len() {
                previous_whitespace_index(s, index)
            } else {
                s.len()
            };

            message.lines.push(&s[last_end..prev_whtspc_idx].trim());

            if message.lines.last().unwrap().len() > message.max_len {
                message.max_len = message.lines.last().unwrap().len();
            };

            if index >= s.len() {
                break;
            }

            last_end = prev_whtspc_idx;
            index = prev_whtspc_idx + WRAP_INDEX;
        }
    }

    message
}
