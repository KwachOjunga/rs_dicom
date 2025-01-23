use dicom::dump::dump_file_to;
use dicom::object::{open_file, DefaultDicomObject};
use dicom::pixeldata::Error;
use dicom::pixeldata::PixelDecoder;
use std::path::PathBuf;

// [TODO] handle errors gracefully
fn _read_file_to_memory(file: PathBuf) -> Option<DefaultDicomObject> {
    if file.as_path().exists() {
        let dcm_file = open_file(file).inspect_err(|e| eprintln!("File opening failed:{e}"));
        if dcm_file.is_ok() {
            dcm_file.ok()
        } else {
            None
        }
    } else {
        None
    }
}

// [TODO] reproduce the bug in this function when it's handling a file whose value representation is altered.
pub fn show_number_of_images(file: PathBuf) -> Result<(DefaultDicomObject, u32), Error> {
    let file = _read_file_to_memory(file).inspect(|e| eprintln!("Check if the file exists:{e:?}")).unwrap();
    let images = file
        .decode_pixel_data()
        .inspect_err(|e| eprintln!("operation failed: {e}"));
    if images.is_ok() {
        let num = images?.number_of_frames();
        Ok((file, num))
    } else {
        Err(images.err().unwrap())
    }
}

// [TODO] generate image from the frame number and save it in either jpg or png.
pub fn dump_pixel_data_of_an_image(file: PathBuf, img_ind: u32) {
    let file_name = file.clone().into_os_string().into_string().unwrap();
    let (file, num) = show_number_of_images(file.into()).unwrap();
    if num < img_ind {
        println!("That index is out of index range. Current number of frames is {num}");
        ()
    } else {
        let file = file.decode_pixel_data().unwrap();
        let img = file.to_dynamic_image(img_ind - 1).unwrap();
        let v_filename: Vec<&str> = if cfg!(windows) {
            file_name.split('\\').collect()
        } else {
            file_name.split('/').collect()
        };
        img.save(format!(
            "{}_{}.png",
            v_filename[v_filename.len() - 1],
            img_ind
        ))
        .unwrap();
        ()
    }
}

//dump entire file metadata on screen
pub fn display_metadata(file: PathBuf) {
    let file_name = file.clone().into_os_string().into_string().unwrap();
    let vec_file_name: Vec<&str> = file_name.split('/').collect();
    let file = _read_file_to_memory(file);
    let output_file =
        std::fs::File::create(format!("{}.txt", vec_file_name[vec_file_name.len() - 1])).unwrap();
    if file != None {
        dump_file_to(output_file, &file.unwrap()).unwrap();
    } else {
        println!("Check if the file exists");
    }
}
