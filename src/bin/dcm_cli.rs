//! This file illustrates various ways of manipulating  a dicom file via a command line interface.
use clap::Parser;
use dicom::pixeldata::Error;
use rs_dicom::{display_metadata, dump_pixel_data_of_an_image, show_number_of_images};

#[derive(Parser, Debug)]
#[command(name = "dcm_cli")]
#[command(version = "0.1")]
#[command(about = "Probe dicom files")]
struct Args {
    /// file to display
    file: Vec<String>,

    /// Specific image frame to extract as png
    #[arg(short, long, value_name = "frame_number(s)")]
    image_to_dump: Option<Vec<u32>>,

    /// extract all images to a directory
    #[arg(short, long, value_name = "true/false")]
    extract: Option<bool>,

    /// Diplay number of images in the file
    #[arg(short, long, value_name = "true/false")]
    list: Option<bool>,

    /// Dump entire file metadata on screen
    #[arg(long, short)]
    dump: Option<bool>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let file = args.file.clone();
    if let Some(val) = args.list.or(None) {
        if val {
            for i in &file {
                println!("{}", i.clone());
                let (_, num) = show_number_of_images(i.clone().into())?;
                println!("{}", num);
            }
        }
    }

    if let Some(dump) = args.dump.or(None) {
        if dump {
            for i in &file {
                display_metadata(i.clone().into());
            }
        }
    }

    //[TODO] there must be a bug in this return type
    match args.image_to_dump {
        Some(ref num) => {
		let total = num.clone();
            for frame in total {
                for i in &file {
                    dump_pixel_data_of_an_image(i.as_str().into(), frame);
                }
            }
            ()
        }
        _ => (),
    }

    match args.extract {
        Some(extract) => {
            if extract {
                for i in &file {
                    let (_, num) = show_number_of_images(i.as_str().into())?;
                    for ind in 1..=num {
                        dump_pixel_data_of_an_image(i.clone().into(), ind);
                    }
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
