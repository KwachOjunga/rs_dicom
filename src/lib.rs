use dicom::dump::dump_file_to;
use dicom::object::{open_file, DefaultDicomObject};
use dicom::pixeldata::PixelDecoder;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};


// [TODO] handle errors gracefully
fn _read_file_to_memory(file: PathBuf) -> Option<DefaultDicomObject> {
    if file.as_path().exists() {
        let dcm_file = open_file(file).unwrap();
        Some(dcm_file)
    } else {
        None
    }
}

pub fn show_number_of_images(file: PathBuf) -> u32 {
    let file = _read_file_to_memory(file).expect("Check if the file exists.");
    let images = file.decode_pixel_data().unwrap();
    images.number_of_frames()
}

//generate image from the frame number and delete it afterwards.
pub fn dump_pixel_data_of_an_image(img_ind: u32) {}

//dump entire file metadata on screen
// [TODO] better to dump the data to a file rather than he screen
pub fn display_metadata(file: PathBuf) {
	let file = _read_file_to_memory(file);
	let output_file = std::fs::File::create("metadata.txt").unwrap();
	if file != None {
			dump_file_to(output_file,&file.unwrap()).unwrap();
		}else{ println!("Check if the file exists");}
	
}
