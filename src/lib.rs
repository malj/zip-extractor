use std::io::{self, BufReader, BufWriter, Read, Seek, Write};
use zip::ZipArchive;

/// Attempts to read the source as a ZIP archive, extract a file,
/// and output its contents to a provided writeable sink.
/// Both read and write operations are chunked
/// to minimize the number of system calls.
///
/// # Arguments
///
/// - `source` - Readable ZIP archive input.
/// - `filename` - Name of the file to be extracted.
/// - `buffer_size` - Size of the read and write buffers in bytes.
/// - `sink` - Writeable output for the extracted file bytes.
pub fn extract(
	source: impl Read + Seek,
	filename: &str,
	buffer_size: usize,
	sink: impl Write,
) -> io::Result<()> {
	let mut archive = ZipArchive::new(source)?;
	let file = archive.by_name(filename)?;
	let reader = BufReader::with_capacity(buffer_size, file);
	let mut writer = BufWriter::with_capacity(buffer_size, sink);

	for byte in reader.bytes() {
		writer.write_all(&[byte?])?;
	}

	writer.flush()
}

#[cfg(test)]
mod test {
	use std::io::{self, Cursor, Read, Seek, Write};

	use zip::{write::SimpleFileOptions, ZipWriter};

	/// Creates a compressed ZIP archive in memory with the following files:
	/// ```txt
	/// .
	/// ├── file.txt -> "outer"
	/// └── nested
	///     └── file.txt -> "inner"
	/// ```
	fn create_archive() -> io::Result<impl Read + Seek> {
		let mut buffer = Cursor::new(Vec::new());
		let mut zip = ZipWriter::new(&mut buffer);
		let options =
			SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

		zip.start_file("file.txt", options)?;
		zip.write_all(b"outer")?;

		zip.add_directory("nested", options)?;

		zip.start_file("nested/file.txt", options)?;
		zip.write_all(b"inner")?;

		zip.finish()?;

		Ok(buffer)
	}

	#[test]
	fn read_file() {
		let source = create_archive().unwrap();
		let mut sink = Vec::new();
		super::extract(source, "file.txt", 8192, &mut sink).unwrap();
		let content = String::from_utf8(sink).unwrap();

		assert_eq!(content, "outer");
	}

	#[test]
	fn read_file_in_dir() {
		let source = create_archive().unwrap();
		let mut sink = Vec::new();
		super::extract(source, "nested/file.txt", 8192, &mut sink).unwrap();
		let content = String::from_utf8(sink).unwrap();

		assert_eq!(content, "inner");
	}

	#[test]
	fn file_not_found() {
		let source = create_archive().unwrap();
		let mut sink = Vec::new();
		let error = super::extract(source, "waldo", 8192, &mut sink).unwrap_err();

		assert_eq!(error.kind(), io::ErrorKind::NotFound);
	}

	#[test]
	fn buffer_overflow() {
		let source = create_archive().unwrap();
		let mut sink = [0u8; 3];
		let error = super::extract(source, "file.txt", 8192, &mut sink[..]).unwrap_err();
		let content = String::from_utf8(sink.to_vec()).unwrap();

		// The buffer was too small for the file content
		assert_eq!(error.kind(), io::ErrorKind::WriteZero);

		// The buffer was only partially written
		assert_eq!(content, "out");
	}
}
