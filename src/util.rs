use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use std::{hash::Hash, str::FromStr};

use crate::error::{CGroupError, Result};
use crate::FlatKeyedSetter;
use std::collections::HashMap;

pub fn read_file_into_string(path: &Path) -> Result<String> {
    match std::fs::File::open(path) {
        Ok(mut file) => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Ok(_) => Ok(buf),
                Err(err) => Err(CGroupError::FSErr(err.kind())),
            }
        }
        Err(err) => Err(CGroupError::FSErr(err.kind())),
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
    content
        .split('\n')
        .map(|s| T::from_str(s))
        .filter_map(std::result::Result::ok)
        .collect()
}

pub fn read_single_value<T: FromStr>(parent: &Path, filename: &str) -> Result<T> {
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    if let Some(w) = content.split('\n').next() {
        let w = T::from_str(w).map_err(|_| CGroupError::UnknownFieldErr(content))?;
        return Ok(w);
    }
    Err(CGroupError::UnknownFieldErr(content))
}

pub fn read_value<T: FromStr>(parent: &Path, filename: &str) -> Result<T> {
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    Ok(T::from_str(content.as_str()).map_err(|_| CGroupError::UnknownFieldErr(content))?)
}

pub fn write_single_value<T: ToString>(parent: &Path, filename: &str, t: T) -> Result<()> {
    let mut path = PathBuf::from(parent);
    path.push(filename);
    fs::write(path.as_path(), t.to_string()).map_err(|e| CGroupError::FSErr(e.kind()))
}

pub fn read_flat_keyed_file<V, T>(parent: &Path, filename: &str) -> Result<T>
where
    V: FromStr,
    T: FlatKeyedSetter<V>,
{
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    let mut t = T::new();
    let mut splits = content.split('\n');
    while let Some(line) = splits.next() {
        if line.is_empty() {
            break;
        }
        let mut kv = line.split_whitespace();
        let key = kv
            .next()
            .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let val = kv
            .next()
            .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let val =
            V::from_str(val).map_err(|_| CGroupError::UnknownFieldErr(content.to_string()))?;
        t.set(key, val);
    }
    Ok(t)
}

pub fn read_flat_keyed_file_map<V>(parent: &Path, filename: &str) -> Result<HashMap<String, V>>
where
    V: FromStr,
{
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    let mut map = HashMap::new();
    let mut splits = content.split('\n');
    while let Some(line) = splits.next() {
        if line.is_empty() {
            break;
        }
        let mut kv = line.split_whitespace();
        let key = kv
            .next()
            .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let val = kv
            .next()
            .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let val =
            V::from_str(val).map_err(|_| CGroupError::UnknownFieldErr(content.to_string()))?;
        map.insert(key.to_string(), val);
    }
    Ok(map)
}

pub fn read_nested_keyed_file_to_map<K1, K2, V2>(
    parent: &Path,
    filename: &str,
) -> Result<HashMap<K1, HashMap<K2, V2>>>
where
    K1: FromStr + Eq + Hash,
    K2: FromStr + Eq + Hash,
    V2: FromStr,
{
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    let mut map = HashMap::new();
    let mut lines = content.split('\n');
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut splits = line.split_whitespace();
        if let Some(first) = splits.next() {
            let k1 = K1::from_str(first)
                .map_err(|_| CGroupError::UnknownFieldErr(content.to_string()))?;
            let mut nest_map = HashMap::new();
            while let Some(next) = splits.next() {
                let mut kv_split = next.split('=');
                let k = kv_split
                    .next()
                    .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
                let v = kv_split
                    .next()
                    .ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
                let k = K2::from_str(k)
                    .map_err(|_| CGroupError::UnknownFieldErr(content.to_string()))?;
                let v = V2::from_str(v)
                    .map_err(|_| CGroupError::UnknownFieldErr(content.to_string()))?;
                nest_map.insert(k, v);
            }
            map.insert(k1, nest_map);
        }
    }

    Ok(map)
}

pub fn read_nested_keyed_file<K, V>(parent: &Path, filename: &str) -> Result<HashMap<K, V>>
where
    K: FromStr<Err=CGroupError> + Eq + Hash,
    V: FromStr<Err=CGroupError>,
{
    let mut path = PathBuf::from(parent);
    path.push(filename);
    let content = read_file_into_string(&path)?;
    let mut map = HashMap::new();
    let mut lines = content.split('\n');
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut kv = line.splitn(2, ' ');
        let k = kv.next().ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let v = kv.next().ok_or(CGroupError::UnknownFieldErr(content.to_string()))?;
        let k = K::from_str(k)?;
        let v = V::from_str(v)?;
        map.insert(k, v);
    }
    Ok(map)
}
