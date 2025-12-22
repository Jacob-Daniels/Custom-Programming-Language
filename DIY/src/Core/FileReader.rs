
// Import standard library
use std::{
	fs::File,
	io::{self, prelude::*},
};

pub struct FileReader {
	m_reader: io::BufReader<File>,
}

impl FileReader {
	pub fn open_file(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
		let _file = File::open(path).expect("Could not find file");
		let m_reader = io::BufReader::new(_file);

		Ok(Self { m_reader } )
	}
	// Option<> allows a generic type to be returned (None or Some([value]).
	pub fn read_line<'buf>(&mut self, buffer: &'buf mut String) -> Option<io::Result<&'buf mut String>> {
		buffer.clear();

		self.m_reader
			.read_line(buffer)
			.map(|var| if var == 0 { None } else { Some(buffer) }) // An iterator to loop the line
			.transpose()
	}
}