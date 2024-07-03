use std::{
    fs::{self, create_dir},
    io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudibleDataLocation {
    path: PathBuf,
}

pub fn copy_dir(src: &Path, dest: &Path, overwrite: bool) -> io::Result<()> {
    // validate that the src directy is not nested
    for entry in src.read_dir()? {
        let entry = entry?;
        if entry.path().is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "src directory must not be nested",
            ));
        }
    }
    fs::create_dir_all(dest)?;
    copy_contents(src, dest, overwrite)
}

pub fn copy_contents(src: &Path, dest: &Path, overwrite: bool) -> io::Result<()> {
    for entry in src.read_dir()? {
        let entry = entry?;

        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        let new_path = dest.join(file_name);

        if entry_path.is_dir() {
            if !new_path.exists() {
                fs::create_dir(&new_path)?;
            }

            copy_contents(&entry_path, &new_path, overwrite)?;
        } else {
            if new_path.exists() && !overwrite {
                continue;
            }

            fs::copy(&entry_path, &new_path)?;
        }
    }

    Ok(())
}

pub trait PathExt {
    fn exists_or_none(self) -> Option<PathBuf>;
}

impl PathExt for PathBuf {
    fn exists_or_none(self) -> Option<PathBuf> {
        match self.exists() {
            true => Some(self),
            false => None,
        }
    }
}

pub fn create_dir_if_not_exists(path: &PathBuf) -> io::Result<()> {
    if !path.exists() {
        match create_dir(&path) {
            Ok(_) => {
                println!("Directory {} created successfully.", path.display());
            }
            Err(e) => {
                // Handle different kinds of errors differently if needed
                panic!("Failed to create directory: {:?}", e);
            }
        }
    } else {
        println!("Directory {} already exists.", path.display());
    }
    Ok(())
}
