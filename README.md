# dcm_cli

A minimal commandline application to interact with dicom files. 
I intend to extend  to be used with other medical file formats.

## Installation

To use the binary cli ensure you have rust installed.

Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` to install.

Clone the repo and `cd rs_dicom` into the directory
Run `cargo install --bin dcm_cli --path=.` ro create the binary file.
To use, view the options available via
`dcm_cli -h`

