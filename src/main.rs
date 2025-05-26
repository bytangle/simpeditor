use core::textviewer2::TextViewer;
use std::{path::Path};

mod core;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		println!("Please provide file name as argument");

		std::process::exit(0);
	}

	let file_path = Path::new(args.get(1).unwrap());

	if !file_path.exists() {
		println!("File does not exist");
		std::process::exit(0);
	}

	println!("{}", termion::cursor::Show);

	// initialize viewer
	let mut viewer = TextViewer::init(file_path);
	viewer.show_document();
	viewer.run();
}
