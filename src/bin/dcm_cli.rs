//! The following program allow one to perform a set of operations on dicom files
//!
//! To use
//!   ```
//!      dcm_cli -h
//!    ```
use clap::Parser;
use rs_dicom::error;
use rs_dicom::{
    display_metadata, dump_pixel_data_of_an_image, dump_pixeldata_of_multiple_images,
    show_number_of_images, view_image_using_opencv 
};

#[derive(Parser, Debug)]
#[command(name = "dcm_cli")]
#[command(version = "0.1")]
#[command(about = "Probe dicom files")]
struct Args {
    /// file to display
    file: Vec<String>,

    /// Specific image frame to extract as png
    #[arg(short, long, value_name = "frame_number")]
    image_to_dump: Vec<u32>,

    /// extract all images to a directory
    #[arg(short, long)]
    extract: bool,

    /// Diplay number of images in the file
    #[arg(short, long)]
    list: bool,

    /// Dump entire file metadata to a txt file of a similar name
    #[arg(long, short, value_name=None)]
    dump: bool,

    /// permit the viewing of an image that has been dumped in the current directory
    #[arg(short, long, value_name = "frame_number")]
    view: Option<u32>,
}

fn main() -> Result<(), error::CliError> {
    let args = Args::parse();
    let file = args.file.clone();
    let images_option = if !args.image_to_dump.is_empty() {
        args.image_to_dump
    } else {
        vec![]
    };
    let _length = images_option.len();

    if args.list {
        for i in &file {
            println!("{}", i.clone());
            let (_, num) = show_number_of_images(i.clone().into())?;
            println!("{}", num);
        }
    }

    if args.dump {
        for i in &file {
            display_metadata(i.clone().into());
        }
    }

    if args.view.is_some() {
        if let Some(ind) = args.view{
            for i in &file {
                view_image_using_opencv(i.clone().into(), ind);
            }
        }
    }
    //[TODO] there must be a bug in this return type
    // match &args.image_to_dump {
    //   Some( num) => {
    //     	let length = num.len();
    // 	for i in &file {
    //       if length > 1 {
    //         dump_pixeldata_of_multiple_images(i.clone().into(), num);
    //   } else {
    //     dump_pixel_data_of_an_image(i.clone().into(), num[0]);
    //  }
    // }
    // ()
    // }
    //  _ => (),
    // }

    // if length > 1 {
    //     for i in &file {
    //         let i = i.clone();
    //         let _ = dump_pixeldata_of_multiple_images(i.into(), &images_option);
    //     }
    // } else if length == 1 {
    //     for i in &file {
    //         let _ = dump_pixel_data_of_an_image(i.clone().into(), images_option[0]);
    //     }
    // } else {
    //     ()
    // }

    if args.extract {
        for i in &file {
            let (_, num) = show_number_of_images(i.as_str().into()).unwrap();
            for ind in 1..=num {
                let _ = dump_pixel_data_of_an_image(i.clone().into(), ind);
            }
        }
        Ok(())
    } else {
        Ok(())
    }
}
