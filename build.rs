use std::fs::File;
use std::io::prelude::*;
use std::{env, fs};

macro_rules! p {
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                println!("cargo:warning={:?}", &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

fn main() {
    // Define the paths to the source and output files
    let manifest_dir = p!(env::var("CARGO_MANIFEST_DIR")).expect("Couldn't get manifest dir");
    let source_file_path = format!("{}/ignored_mods.txt", manifest_dir);

    // let cargo_build_target_dir = p!(env::var("CARGO_TARGET_DIR")).expect("No Cargo target dir");
    let cargo_build_target_dir = format!("{manifest_dir}/target");
    let profile = p!(env::var("PROFILE")).expect("No profile received");
    let output_dir = format!("{}/{}", cargo_build_target_dir, profile);

    let output_file_path = format!("{}/ignored_mods.txt", output_dir);

    // Read content from the source file
    let source_content =
        fs::read_to_string(&source_file_path).expect("Failed to read source file content");

    // Check if the file contains so no unnecessary fs operations happen.
    if let Ok(metadata) = fs::metadata(&output_file_path) {
        if metadata.is_file() {
            if let Ok(output_content) = fs::read_to_string(&output_file_path) {
                if source_content == output_content {
                    return;
                }
            }
        } else {
            return;
        };
    }

    // Write the source file content to the output file
    let mut output_file = File::create(&output_file_path).expect("Unable to create output file");
    write!(output_file, "{}", source_content).expect("Failed to write content to output file");

    // Print a message indicating the file creation
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ignored_mods.txt");
    println!("cargo:rerun-if-env-changed=OUT_DIR");
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
}
