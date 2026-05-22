use clap::Parser; 

#[derive(Parser)]
struct Cli {
	/// Input file path
	path: std::path::PathBuf, 
}

fn main() {
	let args = Cli::parse(); 
	
	println!("path: {:?}", args.path)
}
