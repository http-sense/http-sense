use std::path::PathBuf;
use std::{process::Command}; 


use std::env;

use fs_extra::dir::CopyOptions;


// Example custom build script.
fn main() {
    let out_dir_str = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir_str);
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=frontend/");
    // Use the `cc` crate to build a C file and statically link it.
    // let output = if cfg!(target_os = "windows") {

    let _output = Command::new("bash")
            .args(["-c", "cd frontend && npm i && build_dir=temporary npm run build"])
            .output()
            .expect("failed to execute process");

    if !_output.status.success() {
        panic!("couldn't run command successfully:  {:?}", _output);
    }
    fs_extra::dir::move_dir("frontend/temporary", out_dir.join("frontend_build"), &CopyOptions::default()).unwrap();
    //} else {
    //    Command::new("sh")
    //            .arg("-c")
    //            .arg("echo hello")
    //            .output()
    //            .expect("failed to execute process")
    //};

}
