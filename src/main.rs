//! This file illustrates various ways of manipulating  a dicom file via a command line interface.
use clap::Parser;
use rs_dicom::show_number_of_images;
use rs_dicom::display_metadata;


#[derive(Parser, Debug)]
#[command(name="dcm_cli")]
#[command(
    version="0.1",
    about="cli to display information in dicom files",
    long_about=None
)]
struct Args {
    /// file to display
    file: Vec<String>,

    /// Specific image frame to dump
    #[arg(short, long, value_name="image_frame")]
    image_to_dump: Option<u8>,

	/// Diplay number of images in the file
	#[arg(short, long)]
	list: Option<bool>,

	/// Dump entire file metadata on screen
	#[arg(long, short, default_value="false")]
	dump: Option<bool>
	 
}


//parse arguments and pass them to functions from lib.rs

//
fn main() {
    let args = Args::parse();
	let file = args.file;
	if let Some(val) = args.list.or(None) {
		if val {
			for i in &file {
				println!("{}", i.clone());
				println!("{}", show_number_of_images(i.clone().into()));
			}
			//println!("{}",show_number_of_images(file.clone().into()));
		}
	}
	
	if let Some(dump) = args.dump.or(None) {
		if dump { 
			for i in &file {
			display_metadata(i.clone().into()); 
			}
	}

}
}
