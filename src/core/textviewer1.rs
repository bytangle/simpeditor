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

struct TextViewer {
  doc: Doc,
  doc_length: usize,
  cursor_position: Coordinates,
  terminal_size: Size,
  file_name: String,
}