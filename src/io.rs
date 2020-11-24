use std::{str::FromStr, collections::HashMap};
use std::path::Path;

use crate::{error::{CGroupError, Result}, util::read_nested_keyed_file};
#[derive(Debug, Eq, PartialEq)]
pub struct IO<'a> {
    path: &'a Path
}

impl<'a> IO<'a> {
    pub fn new(path: &'a Path) -> IO {
        IO {
            path
        }
    }

    pub fn stat(&self) -> Result<HashMap<DeviceNumber, Stat>> {
        let filename = "memory.stat";
        read_nested_keyed_file(&self.path, filename)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DeviceNumber {
    pub maj: u32,
    pub min: u32
}

impl FromStr for DeviceNumber {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<DeviceNumber> {
        let mut splits = s.split(':');
        let maj = splits.next()
            .ok_or(CGroupError::UnknownFieldErr(s.to_string()))?;
        let min = splits.next()
            .ok_or(CGroupError::UnknownFieldErr(s.to_string()))?;
        Ok(DeviceNumber {
            maj: u32::from_str(maj).map_err(|_| CGroupError::UnknownFieldErr(s.to_string()))?,
            min: u32::from_str(min).map_err(|_| CGroupError::UnknownFieldErr(s.to_string()))?
        })
    }
}

pub struct Stat {
    pub rbytes: u32,
    pub wbytes: u32,
    pub rios: u32,
    pub wios: u32,
    pub dbytes: u32,
    pub dios: u32
}

impl Stat {
    fn set(&mut self, k: &str, v: &str) -> Result<()> {
        let v = u32::from_str(v)
            .map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?;
        match k {
            "rbytes" => self.rbytes = v,
            "wbytes" => self.wbytes = v,
            "rios" => self.rios = v,
            "wios" => self.wios = v,
            "dbytes" => self.dbytes = v,
            "dios" => self.dios = v,
            _ => {}
        }
        Ok(())
    }
}

impl FromStr for Stat {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<Self> {
        let mut stat = Stat {
           rbytes: 0,
            wbytes: 0,
            rios: 0,
            wios: 0,
            dbytes: 0,
            dios: 0
        };
        let mut splits = s.split_ascii_whitespace();
        while let Some(next) = splits.next() {
            let mut kv = next.split('=');
            let key = kv.next().ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = kv.next().ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            stat.set(key, val)?;
        }
        Ok(stat)
    }
}