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
    #[arg(short, long, value_name = "image_frame_number")]
    image_to_dump: Option<u32>,

    /// extract all images to a directory
    #[arg(short, long, value_name = "extract_all_images")]
    extract: Option<bool>,

    /// Diplay number of images in the file
    #[arg(short, long)]
    list: Option<bool>,

    /// Dump entire file metadata on screen
    #[arg(long, short, default_value = "false")]
    dump: Option<bool>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let file = args.file;
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
        Some(num) => {
            dump_pixel_data_of_an_image(file[0].clone().into(), num);
            ()
        }
        _ => (),
    }


    match args.extract {
        Some(val) => {
            if val {
                for i in &file {
                    let (_, num) = show_number_of_images(i.clone().into())?;
                    for ind in 0..num {
                        dump_pixel_data_of_an_image(i.clone().into(), ind);
                    }
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
