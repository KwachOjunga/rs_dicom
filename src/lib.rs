use dicom::dump::dump_file_to;
use dicom::object::{open_file, DefaultDicomObject};
use dicom::pixeldata::PixelDecoder;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Stdio};
pub mod error;
use opencv::{core, highgui, imgcodecs, prelude::*};

// #[cfg(target = "unix" )]
fn _check_whether_file_is_dicom(file: &PathBuf) -> error::Result<bool> {
    if file.as_path().exists() {
        let n_file = file.as_path().as_os_str().to_str().unwrap();
        let command = Command::new("file")
            .arg(n_file)
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");
        let cmd_out = String::from_utf8_lossy(&command.stdout);
        if cmd_out.contains("DICOM medical imaging data") {
            Ok(true)
        } else {
            Err(std::io::Error::from(ErrorKind::Unsupported).into())
        }
    } else {
        Err(std::io::Error::from(ErrorKind::NotFound).into())
    }
}

// [TODO] handle errors gracefully -- this None value ought not be returned arbitrarily
fn _read_file_to_memory(file: PathBuf) -> error::Result<DefaultDicomObject> {
    let file_nat = _check_whether_file_is_dicom(&file)?;
    if file_nat != true {
        panic!("Not a dicom file");
    } else {
        let dcm_file = open_file(file).inspect_err(|e| eprintln!("File opening failed:{e}"));
        if dcm_file.is_ok() {
            Ok(dcm_file?)
        } else {
            Err(dcm_file.expect_err("file is not ok").into())
        }
    }
}

// [TODO] reproduce the bug in this function when it's handling a file whose value representation is altered.
pub fn show_number_of_images(file: PathBuf) -> error::Result<(DefaultDicomObject, u32)> {
    let file = _read_file_to_memory(file);
    match file {
        Ok(dcm_file) => {
            let images = dcm_file
                .decode_pixel_data()
                .inspect_err(|e| eprintln!("operation failed:{e}"));
            if images.is_ok() {
                let num = images?.number_of_frames();
                Ok((dcm_file, num))
            } else {
                Err(images.err().unwrap().into())
            }
        }
        _ => Err(file.unwrap_err()),
    }
}

// [TODO] generate image from the frame number and save it in either jpg or png.
pub fn dump_pixel_data_of_an_image(file: PathBuf, img_ind: u32) -> error::Result<()> {
    let file_name = file.clone();
    let file_name = file_name.as_os_str().to_str().unwrap();
    let (file, num) = show_number_of_images(file.into())?;
    if num < img_ind {
        println!("That index is out of index range. Current number of frames is {num}");
        Ok(())
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
        Ok(())
    }
}
pub fn dump_pixeldata_of_multiple_images(file: PathBuf, img_inds: &Vec<u32>) {
    let file_name = file.clone();
    let file_name = file_name.as_os_str().to_str().unwrap();
    let (file, num) = show_number_of_images(file_name.into()).unwrap();
    let mut container = vec![];
    for ind in img_inds {
        if &num < ind || ind == &0 {
            println!("The frame number {ind} is out of bounds for the file {file_name}");
        } else {
            container.push(ind);
        }
    }
    for frame in container {
        let file = file.decode_pixel_data().unwrap();
        let img = file.to_dynamic_image(*frame - 1).unwrap();
        let img_filename: Vec<&str> = if cfg!(windows) {
            file_name.split('\\').collect()
        } else {
            file_name.split('/').collect()
        };
        img.save(format!(
            "{}_{}.png",
            img_filename[img_filename.len() - 1],
            frame
        ))
        .unwrap()
    }
}

//dump entire file metadata on screen
pub fn display_metadata(file: PathBuf) {
    let file_name = file.clone().into_os_string().into_string().unwrap();
    let vec_file_name: Vec<&str> = file_name.split('/').collect();
    let file = _read_file_to_memory(file);
    let output_file =
        std::fs::File::create(format!("{}.txt", vec_file_name[vec_file_name.len() - 1])).unwrap();
    if file.is_ok() {
        dump_file_to(output_file, &file.unwrap()).unwrap();
    } else {
        //println!("Check if the file exists");
        let _ = file.inspect_err(|e| eprintln!("Failed to read file: {e}"));
    }
}

// pub fn _yield_path_name(file: PathBuf, img_ind: u32) -> error::Result<()> {
//     let file_name = file.clone();
//     let file_name = file_name.as_os_str().to_str().unwrap();
//     let (file, num) = show_number_of_images(file.into())?;
//     if num < img_ind {
//         println!("That index is out of index range. Current number of frames is {num}");
//         Ok(())
//     } else {
//         let file = file.decode_pixel_data().unwrap();
//         let img = file.to_dynamic_image(img_ind - 1).unwrap();
//         let v_filename: Vec<&str> = if cfg!(windows) {
//             file_name.split('\\').collect()
//         } else {
//             file_name.split('/').collect()
//         };
//         let path = format!("{}_{}.png", v_filename[v_filename.len() - 1], img_ind);
//         img.save(path).unwrap();
// }

pub fn view_image_using_opencv(file: PathBuf, img_ind: u32) /*-> error::Result<()>*/
{
    let file_name = file.clone();
    let file_name = file_name.as_os_str().to_str().unwrap();
    let (file, num) = show_number_of_images(file.into()).unwrap();
    if num < img_ind {
        println!("That index is out of index range. Current number of frames is {num}");
    } else {
        let file = file.decode_pixel_data().unwrap();
        let img = file.to_dynamic_image(img_ind - 1).unwrap();
        let v_filename: Vec<&str> = if cfg!(windows) {
            file_name.split('\\').collect()
        } else {
            file_name.split('/').collect()
        };
        let path = format!("{}_{}.png", v_filename[v_filename.len() - 1], img_ind);
        img.save(&path).unwrap();

        // Read image (IMREAD_COLOR loads a 3-channel color image)
        let img = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR).unwrap();

        if img.empty() {
            eprintln!("Failed to read image at '{}'", path);
            std::process::exit(1);
        }

        // Create a named window (flags: 0 -> autosize; highgui::WINDOW_AUTOSIZE can be used explicitly)
        highgui::named_window("display", highgui::WINDOW_AUTOSIZE).unwrap();

        // Show the image in the window
        highgui::imshow("display", &img).unwrap();

        // waitKey(0) waits indefinitely until a key is pressed.
        // Note: wait_key is necessary for GUI housekeeping and to actually display the window.
        highgui::wait_key(0).unwrap();

        // Optionally destroy all windows
        highgui::destroy_all_windows().unwrap();

        // Ok(())
    }
}
