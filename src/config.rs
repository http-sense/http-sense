
use std::{fs::{self, File}, path::{Path, PathBuf}};

use anyhow::Context;
use directories;

pub fn get_data_dir() -> anyhow::Result<PathBuf> {
    let b = directories::ProjectDirs::from("", "", "http-sense")
        .context("This operating system has no home directory")?;
    let local_dir = b.data_local_dir();
    fs::create_dir_all(local_dir)?;
    Ok(local_dir.to_path_buf())
}

#[cfg(not(debug_assertions))]
pub fn get_database_file() -> anyhow::Result<String> {
    let data_dir = get_data_dir()?;
    let database_file_path = data_dir.join("history.sqlite");
    // I want a way to create new file while knowing if the error is because file already exists or an IO::Error
    let create_new_res = File::create_new(&database_file_path);
    if File::open(&database_file_path).is_err() {
        match create_new_res {
            Err(x) => {
                return Err(x.into());
            },
            Ok(_) => unreachable!() // 
        }
    }

    Ok(format!("sqlite://{}", dbg!(database_file_path).to_str().context("Invalid Database Dir Path")?))
}

#[cfg(debug_assertions)]
pub fn get_database_file() -> anyhow::Result<String> {
    let database_file_path = "dev.sqlite";
    // I want a way to create new file while knowing if the error is because file already exists or an IO::Error
    let create_new_res = File::create_new(&database_file_path);
    if File::open(&database_file_path).is_err() {
        match create_new_res {
            Err(x) => {
                return Err(x.into());
            },
            Ok(_) => unreachable!() // 
        }
    }

    Ok(format!("sqlite://{}", dbg!(database_file_path)))
}
