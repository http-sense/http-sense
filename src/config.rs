
use std::{fs::{self, File}, path::{PathBuf}};

use anyhow::Context;
use directories;



pub const SUPABASE_PROJECT_URL: &'static str = "https://wfeoffbfmtjjzamwjlob.supabase.co";
pub const SUPABASE_ANON_KEY: &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6IndmZW9mZmJmbXRqanphbXdqbG9iIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzA4NjU3ODMsImV4cCI6MTk4NjQ0MTc4M30.6S-W8RcMx7zhIwAkuukw3kv-0zylHntZbxBopaN6C5s";

pub fn get_data_dir() -> anyhow::Result<PathBuf> {
    let b = directories::ProjectDirs::from("", "", "http-sense")
        .context("This operating system has no home directory")?;
    let local_dir = b.data_local_dir();
    fs::create_dir_all(local_dir)?;
    Ok(local_dir.to_path_buf())
}

// #[cfg(not(debug_assertions))]
// pub fn get_database_file() -> anyhow::Result<String> {
//     let data_dir = get_data_dir()?;
//     let database_file_path = data_dir.join("history.sqlite");
//     // I want a way to create new file while knowing if the error is because file already exists or an IO::Error
//     let create_new_res = File::create_new(&database_file_path);
//     if File::open(&database_file_path).is_err() {
//         match create_new_res {
//             Err(x) => {
//                 return Err(x.into());
//             },
//             Ok(_) => unreachable!() // 
//         }
//     }

//     Ok(format!("sqlite://{}", dbg!(database_file_path).to_str().context("Invalid Database Dir Path")?))
// }

#[cfg(not(debug_assertions))]
pub fn get_database_file() -> anyhow::Result<String> {
    // tempfile::tempdir()
    // tempfile::tempfile();

    use crate::supabase_auth::get_random_string;
    let data_dir = std::env::temp_dir();
    fs::create_dir_all(&data_dir)?;


    
    let file_name = get_random_string(30);
    // let data_dir = get_data_dir()?;
    let database_file_path = data_dir.join(format!("{file_name}.sqlite"));

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
