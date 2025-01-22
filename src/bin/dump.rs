use dicom::dump::dump_file;
use dicom::object::open_file;
use rs_dicom::display_metadata;

fn main() {
	//let file = open_file("/mnt/c/Users/user/Downloads/dicom_viewer_0002/0002.DCM").unwrap();
	//dump_file(&file).unwrap();
	display_metadata("/mnt/c/Users/user/Downloads/dicom_viewer_0002/0002.DCM".into());
}
