use std::fs::{create_dir_all, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;
use std::fs::File;

use crate::rusty_tasks::*;

/// Save tasklist struct to file
pub fn save_tltofile(filepath:String,tasklist:TaskList)->Result<String,Error>{
    let string_tasklist=convert_tltostring(tasklist);

    // println!("Requested path: {}",filepath);// ? debug

    // Create intermediate directories if they don't exist
    if let Some(parent) = std::path::Path::new(&filepath).parent() {
        if let Err(directory_missing_error) = create_dir_all(parent) {
            eprintln!("Error creating directories: {}", directory_missing_error);
            return Err(directory_missing_error);
        }
    };

    // Check if the file exists before handling
    let file_exists = std::path::Path::new(&filepath).exists();

    if file_exists{     
        if let Err(file_exist_error) = handle_existing_file(&filepath,&string_tasklist){
                eprintln!("Invalid handling of existing file:{file_exist_error}")
        }
    }else{
        //println!("not exists");// ? debug
        if let Err(new_file_error) = handle_new_file(&filepath, &string_tasklist){
                eprintln!("Invalid handling of new file:{new_file_error}")
        }
    }
    Ok("File saved successfully.".to_string())
}

/// Non-existing file save
pub fn handle_new_file(filepath: &str, data: &str)->Result<(),Error>{
    // Check if we have write permissions for the folder
    let parent_directory = Path::new(filepath).parent().ok_or_else(|| {
        eprintln!("Error getting parent directory for file: {}", filepath);
        Error::new(ErrorKind::Other, "Invalid parent directory")
    })?;

    let parent_notreadonly=!parent_directory.metadata()?.permissions().readonly();

    if parent_notreadonly { // Folder has write permissions, create file        
        let mut file = File::create(filepath)?;
        file.write_all(data.as_bytes())?;
    } else {
        eprintln!("Error: No write permissions for the new file's parent directory.");
        return Err(Error::new(ErrorKind::PermissionDenied, "No write permissions for parent directory"));
    }

    Ok(())
}

/// Existing file save
pub fn handle_existing_file(filepath: &str, data: &str)->Result<(),Error>{
    let data_directory = std::env::current_dir()?.join("data");
    let data_directory_canon = data_directory.canonicalize()?.to_string_lossy().to_string();
    let filepath_canon = Path::new(filepath).canonicalize()?.to_string_lossy().to_string();

    if !filepath_canon.contains(&data_directory_canon) {
        eprintln!(
            "Error: Path does not contain the 'data' directory: {} datadir: {}",
            filepath_canon, data_directory_canon
        );
        return Err(Error::new(ErrorKind::Other, "Invalid path"));
    }

    let file_exists = Path::new(filepath).exists();
    let not_readonly = !Path::new(filepath).metadata()?.permissions().readonly();

    match (file_exists, not_readonly) {
        (true, true) => {
            // File exists, have permissions, overwrite it
            let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(filepath)?;
            file.write_all(data.as_bytes())?;
        }
        (true, false) => {
            // File exists, no permissions, error
            eprintln!("Error: No write permissions for the existing file.");
            return Err(Error::new(ErrorKind::PermissionDenied, "No write permissions"));
        }
        (false, _) => {
            // File not exists
        }
    }

    Ok(())
}

/// Load tasklist struct from file
pub fn load_tlfromfile(path:String)->TaskList{
    let mut data = String::new();
    let mut file = File::open(&path).ok();
    let mut file_opened=false;
    match file{
        Some(ref mut _f)=>{file_opened=true;}
        None=>{
            #[cfg(debug_assertions)]{//prevent this from running in release
                eprintln!("Error loading: File not found at path '{}'", &path);
            }
        }
    };
    if file_opened{
        file.unwrap().read_to_string(&mut data).unwrap_or_default();
    }
    convert_stringtotl(data)
}