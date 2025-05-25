use std::{fs, io::Write, path::Path};

use termion::{color, event::Key, input::TermRead, raw::IntoRawMode, style};

struct Doc {
  lines: Vec<String>,
}

#[derive(Debug)]
struct Coordinates {
  pub x: usize,
  pub y: usize,
}

#[derive(Debug)]
struct Size {
  pub width: usize,
  pub height: usize,
}

pub struct TextViewer {
  doc: Doc,
  doc_length: usize,
  current_position: Coordinates,
  terminal_size: Size,
  file_name: String,
}

impl TextViewer {
  pub fn init(file_path: &Path) -> Self {
    let mut doc = Doc {
      lines: Vec::new(),
    };

    let file_handle = fs::read_to_string(file_path).unwrap();

    for line in file_handle.lines() {
      doc.lines.push(line.into());
    }

    let doc_length = file_handle.lines().count();

    let size = termion::terminal_size().unwrap();

    Self {
      doc,
      current_position: Coordinates { x:1, y: doc_length },
      doc_length,
      terminal_size: Size { width: size.0 as usize, height: size.1 as usize },
      file_name: file_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string(),
    }
  }

  pub fn show_document(&mut self) {
    let pos = &self.current_position;
    let (old_x, old_y) = (pos.x, pos.y);

    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    println!(
      "{}{}Welcome to Simpeditor text viewer \r{}\n",
      color::Bg(color::Black),
      color::Fg(color::White),
      style::Reset
    );

    for line in 0..self.doc_length {
      println!("{}\r", self.doc.lines[line as usize]);
    }

    println!("{}", termion::cursor::Goto(0, (self.terminal_size.height - 2) as u16));

    println!(
      "{}{} line-count={} Filename: {}{}",
      color::Fg(color::Red),
      style::Bold,
      self.doc_length,
      self.file_name,
      style::Reset,
    );

    self.set_pos(old_x, old_y);
  }

  pub fn run(&mut self) {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdin = std::io::stdin();

    for c in stdin.keys() {
      match c.unwrap() {
        Key::Ctrl('c') => {
          break;
        },
        _ => {}
      }
      stdout.flush().unwrap();
    }
  }

  fn set_pos(&mut self, x: usize, y: usize) {
    self.current_position.x = x;
    self.current_position.y = y;

    println!("{}", termion::cursor::Goto(self.current_position.x as u16, self.current_position.y as u16))
  }
}