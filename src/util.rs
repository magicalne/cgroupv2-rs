use crate::error::{
    CGroupError,
    Result
};
use std::{
    io::Read,
    path::Path
};
use std::str::FromStr;

pub fn read_file_into_string(path: &Path) -> Result<String> {
    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Ok(_) => {
                    Ok(buf)
                },
                Err(err) => {
                    Err(CGroupError::FileSystemFailure(err))
                }
            }
        },
        Err(err) => {
            Err(CGroupError::FileSystemFailure(err))
        }
    }
}

pub fn read_space_separated_values<T: FromStr>(content: String) -> Vec<T> {
    content
        .split_whitespace()
        .into_iter()
        .map(|s| T::from_str(s))
        .filter_map(std::result::Result::ok)
        .collect()
}

pub fn read_newline_separated_values<T: FromStr>(content: String) -> Vec<T>  {
    content.split('\n')
        .map(|s| T::from_str(s))
        .filter_map(std::result::Result::ok)
        .collect()
}

pub fn read_single_value<T: FromStr>(content: String) -> std::result::Result<T, <T>::Err> {
    T::from_str(&content[..])
}