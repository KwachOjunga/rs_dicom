//! This file illustrates various ways of manipulating  a dicom file via a command line interface.
use clap::{Arg, Parser};
use rs_dicom::show_number_of_images;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "cli to display information about dicom files"
)]
struct Args {
    /// file to display
    #[arg(long)]
    file: String,

    /// frame number to dump
    #[arg(short, long)]
    image_to_dump: u8,
}

//parse arguments and pass them to functions from lib.rs

//
fn main() {
    let args = Args::parse();
}
