//! This file illustrates various ways of manipulating  a dicom file via a command line interface.
use clap::Parser;
use dicom::pixeldata::Error;
use rs_dicom::error;
use rs_dicom::{
    display_metadata, dump_pixel_data_of_an_image, dump_pixeldata_of_multiple_images,
    show_number_of_images,
};

#[derive(Parser, Debug)]
#[command(name = "dcm_cli")]
#[command(version = "0.1")]
#[command(about = "Probe dicom files")]
struct Args {
    /// file to display
    file: Vec<String>,

    /// Specific image frame to extract as png
    #[arg(short, long, value_name = "frame_number(s)")]
    image_to_dump: Vec<u32>,

    /// extract all images to a directory
    #[arg(short, long, value_name = "true/false")]
    extract: Option<bool>,

    /// Diplay number of images in the file
    #[arg(short, long, value_name = "true/false")]
    list: Option<bool>,

    /// Dump entire file metadata on screen
    #[arg(long, short, value_name=None)]
    dump: Option<bool>,
}

fn main() -> Result<(), error::CliError> {
    let args = Args::parse();
    let file = args.file.clone();
    let images_option = if !args.image_to_dump.is_empty() {
        args.image_to_dump
    } else {
        vec![]
    };
    let length = images_option.len();

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
    //match &args.image_to_dump {
    //  Some( num) => {
    //    	let length = num.len();
    //	for i in &file {
    //      if length > 1 {
    //        dump_pixeldata_of_multiple_images(i.clone().into(), num);
    //  } else {
    //    dump_pixel_data_of_an_image(i.clone().into(), num[0]);
    // }
    //}
    //()
    //}
    // _ => (),
    //}

    // primary concern remains if the number of frames needed to be generated exceed 1
    if length > 1 {
        for i in &file {
            let i = i.clone();
            let _ = dump_pixeldata_of_multiple_images(i.into(), &images_option);
        }
    } else if length == 1 {
        for i in &file {
            let _ = dump_pixel_data_of_an_image(i.clone().into(), images_option[0]);
        }
    } else {
        ()
    }

    match args.extract {
        Some(extract) => {
            if extract {
                for i in &file {
                    let (_, num) = show_number_of_images(i.as_str().into()).unwrap();
                    for ind in 1..=num {
                        let _ = dump_pixel_data_of_an_image(i.clone().into(), ind);
                    }
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
