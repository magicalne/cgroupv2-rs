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
            let _ = file.read_to_string(&mut buf);
            Ok(buf)
        },
        Err(err) => {
            Err(CGroupError::FileSystemFailure(err))
        }
    }
}

pub fn read_space_separated_values<T: FromStr>(path: &Path)
    -> Result<Vec<std::result::Result<T, <T>::Err>>> {
    let content = read_file_into_string(path)?;
    let splits = content
        .split_whitespace()
        .into_iter()
        .map(|s| T::from_str(s))
        .collect();
    Ok(splits)
}

//TODO return type should be simple
pub fn read_single_value<T: FromStr>(path: &Path) -> Result<std::result::Result<T, <T>::Err>> {
    let content = read_file_into_string(path)?;
    let len = content.len();
    if len > 1 {
        Ok(T::from_str(&content[..len-1]))
    } else {
        Err(CGroupError::UnknownField(content))
    }
}