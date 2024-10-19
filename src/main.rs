use std::{fs::File, io, path::PathBuf};

use clap::Parser;

#[derive(clap::Parser)]
struct Args {
	/// ZIP archive path
	#[arg(short, long)]
	archive_path: PathBuf,

	/// Archived file name
	#[arg(short, long)]
	filename: String,

	/// Read and write buffer sizes in bytes
	#[arg(short, long, default_value_t = 8192)]
	buffer_size: usize,
}

pub fn main() -> io::Result<()> {
	let args = Args::parse();
	let source = File::open(args.archive_path)?;
	let sink = io::stdout();

	zip_extractor::extract(source, &args.filename, args.buffer_size, sink)
}
