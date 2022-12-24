use std::process::Command;


// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=frontend/");
    // Use the `cc` crate to build a C file and statically link it.
    // let output = if cfg!(target_os = "windows") {

    let _output = Command::new("bash")
            .args(["-c", "cd frontend && npm i && npm run build"])
            .output()
            .expect("failed to execute process");

    if !_output.status.success() {
        panic!("couldn't run command successfully:  {:?}", _output);
    }
    //} else {
    //    Command::new("sh")
    //            .arg("-c")
    //            .arg("echo hello")
    //            .output()
    //            .expect("failed to execute process")
    //};

}
