use std::{
    fs,
    io::Read,
    path::{
        Path,
        PathBuf,
    }};
use std::str::FromStr;

use crate::error::{
    CGroupError,
    Result,
};
use crate::FlatKeyedSetter;
use std::collections::HashMap;

pub fn read_file_into_string(path: &Path) -> Result<String> {
    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Ok(_) => {
                    Ok(buf)
                }
                Err(err) => {
                    Err(CGroupError::FSErr(err.kind()))
                }
            }
        }
        Err(err) => {
            Err(CGroupError::FSErr(err.kind()))
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

pub fn read_newline_separated_values<T: FromStr>(content: String) -> Vec<T> {
    content.split('\n')
        .map(|s| T::from_str(s))
        .filter_map(std::result::Result::ok)
        .collect()
}

pub fn read_single_value<T: FromStr>(parent: &Path, filename: &str) -> Result<T> {
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    if let Some(w) = content.split('\n').next() {
        let w = T::from_str(w)
            .map_err(|_| CGroupError::UnknownFieldErr(content))?;
        return Ok(w);
    }
    Err(CGroupError::UnknownFieldErr(content))
}

pub fn read_value<T: FromStr>(parent: &Path, filename: &str) -> Result<T> {
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    Ok(T::from_str(content.as_str())
        .map_err(|_| CGroupError::UnknownFieldErr(content))?)
}

pub fn write_single_value<T: ToString>(parent: &Path, filename: &str, t: T) -> Result<()> {
    let mut path = PathBuf::from(parent);
    path.push(filename);
    fs::write(path.as_path(), t.to_string())
        .map_err(|e| CGroupError::FSErr(e.kind()))
}

pub fn read_flat_keyed_file<V, T>(parent: &Path, filename: &str) -> Result<T>
    where V: FromStr,
          T: FlatKeyedSetter<V>
{
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    let mut t = T::new();
    let mut splits = content.split('\n');
    while let Some(line) = splits.next() {
        if line.is_empty() {
            break
        }
        let mut kv = line.split_whitespace();
        let key = kv.next()
            .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let val = kv.next()
            .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let val = V::from_str(val)
            .map_err(|_| CGroupError::UnknownFieldErr(content.to_string()))?;
        t.set(key, val);
    }
    Ok(t)
}

pub fn read_flat_keyed_file_map<V>(parent: &Path, filename: &str) -> Result<HashMap<String, V>>
    where V: FromStr
{
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    let mut map = HashMap::new();
    let mut splits = content.split('\n');
    while let Some(line) = splits.next() {
        if line.is_empty() {
            break
        }
        let mut kv = line.split_whitespace();
        let key = kv.next()
            .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let val = kv.next()
            .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let val = V::from_str(val)
            .map_err(|_| CGroupError::UnknownFieldErr(content.to_string()))?;
        map.insert(key.to_string(), val);
    }
    Ok(map)
}