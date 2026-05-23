use anyhow::{Context, Result};
use clap::Parser; 
use image::GenericImageView; 

#[derive(Parser)]
struct Cli {
	/// Input file path
	path: std::path::PathBuf, 
}

fn main() -> Result<()> {
	let args = Cli::parse(); 
	
	let input_img = image::open(&args.path).with_context(|| format!("could not read image `{}`", args.path.display()))?; 
	
	println!("path: {:?}", args.path);

	let (input_width, input_height) = input_img.dimensions();
	println!("Loaded image: {}x{}",input_width, input_height);	
	
	let pixel = input_img.get_pixel(0, 0); 
	println!("Top-left pixel RGBA: {:?}", pixel); 

	Ok(())
}
