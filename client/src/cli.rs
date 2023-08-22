use std::{path::PathBuf, io::{BufReader, BufWriter, Cursor, Read, Write}, fs::File};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Input file to convert, don't specify for stdin
	#[arg(short, long)]
	pub infile: Option<Option<PathBuf>>,
	
	/// Format the input file is, is required for stdin
	#[arg(long)]
	pub informat: Option<String>,
	
	/// Path to save the converted file to, don't specify for stdout
	#[arg(short, long)]
	pub outfile: Option<Option<PathBuf>>,
	
	/// Format to convert the input file to, is required for stdout
	#[arg(long)]
	pub outformat: Option<String>,
}

pub fn handle_cli() -> Result<(), Box<dyn std::error::Error>> {
	let args = std::env::args();
	if args.len() == 1 {return Ok(())}
	
	use clap::{Parser, CommandFactory};
	
	let args = Cli::parse_from(args);
	
	let Some(infile) = args.infile else {
		Cli::command()
			.error(clap::error::ErrorKind::MissingRequiredArgument, "--infile is required")
			.exit()
	};
	
	if infile.is_none() && args.informat.is_none() {
		Cli::command()
			.error(clap::error::ErrorKind::MissingRequiredArgument, "--informat is required when using stdin")
			.exit()
	};
	
	let Some(outfile) = args.outfile else {
		Cli::command()
			.error(clap::error::ErrorKind::MissingRequiredArgument, "--outfile is required")
			.exit()
	};
	
	if outfile.is_none() && args.outformat.is_none() {
		Cli::command()
			.error(clap::error::ErrorKind::MissingRequiredArgument, "--outformat is required when using stdout")
			.exit()
	};
	
	let informat = args.informat.unwrap_or_else(|| infile.as_ref().unwrap().to_string_lossy().split('.').last().unwrap().to_string());
	let outformat = args.outformat.unwrap_or_else(|| outfile.as_ref().unwrap().to_string_lossy().split('.').last().unwrap().to_string());
	
	let converter = if let Some(infile) = infile {
		noumenon::Convert::from_ext(&informat, &mut BufReader::new(File::open(infile)?))?
	} else {
		let mut data = Vec::new();
		std::io::stdin().lock().read_to_end(&mut data)?;
		noumenon::Convert::from_ext(&informat, &mut BufReader::new(Cursor::new(data)))?
	};
	
	if let Some(outfile) = outfile {
		if let Some(parent) = outfile.parent() {
			std::fs::create_dir_all(parent)?;
		}
		
		converter.convert(&outformat, &mut BufWriter::new(File::create(outfile)?))?;
	} else {
		let mut data = Vec::new();
		converter.convert(&outformat, &mut BufWriter::new(Cursor::new(&mut data)))?;
		std::io::stdout().lock().write_all(&data)?;
	}
	
	std::process::exit(0)
}