use std::env;
use std::path::PathBuf;
use std::process::Command;

// Example custom build script.
fn main() {
    let out_dir_str = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir_str);
    println!("cargo:rerun-if-changed=frontend/");
    let frontend_build_path = out_dir.join("frontend_build").to_str().unwrap().to_string();
    let frontend_source_path = out_dir.join("frotend_src").to_str().unwrap().to_string();

    let _output = Command::new("bash")
        .args([
            "-c",
            &format!("pwd && rm -rf {frontend_source_path} && cp -r frontend {frontend_source_path} && cd {frontend_source_path} && npm i && build_dir={frontend_build_path} npm run build"),
        ])
        .output()
        .expect("failed to execute process");

    if !_output.status.success() {
        println!("Stdout:\n{}", String::from_utf8_lossy(&_output.stdout));
        println!("Stderr:\n{}", String::from_utf8_lossy(&_output.stderr));
        panic!("couldn't run command successfully: state: {:?}", _output.status);
    }
}
