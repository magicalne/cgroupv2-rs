use std::{
    path::{
        Path,
        PathBuf,
    },
    str::FromStr,
};

use crate::{
    error::{
        CGroupError,
        Result,
    },
    util::read_file_into_string,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Cpu<'a> {
    path: &'a Path
}

impl<'a> Cpu<'a> {
    pub fn new(path: &'a Path) -> Self {
        Cpu {
            path
        }
    }

    pub fn stat(&self) -> Result<Stat> {
        let filename = "cpu.stat";
        let mut path = PathBuf::from(&self.path);
        path.push(filename);
        let content = read_file_into_string(path.as_path())?;
        Stat::from_str(&content)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Stat {
    pub usage_usec: u64,
    pub user_usec: u64,
    pub system_usec: u64,

    //and the following three when the controller is enabled:
    pub nr_periods: u64,
    pub nr_throttled: u64,
    pub throttled_usec: u64,
}

impl Stat {
    fn setter(&mut self, s: &str, val: u64) {
        match s {
            "usage_usec" => self.usage_usec = val,
            "user_usec" => self.user_usec = val,
            "system_usec" => self.system_usec = val,

            "nr_periods" => self.nr_periods = val,
            "nr_throttled" => self.nr_throttled = val,
            "throttled_usec" => self.throttled_usec = val,

            _ => {}
        }
    }
}

impl FromStr for Stat {
    type Err = CGroupError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut stat = Stat {
            usage_usec: 0,
            user_usec: 0,
            system_usec: 0,
            nr_periods: 0,
            nr_throttled: 0,
            throttled_usec: 0,
        };
        let mut splits = s.split('\n');
        while let Some(line) = splits.next() {
            if line.is_empty() {
                break
            }
            let mut kv = line.split_whitespace();
            let key = kv.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = kv.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = u64::from_str(val)
                .map_err(|_| CGroupError::UnknownFieldErr(s.to_string()))?;
            stat.setter(key, val);
        }
        Ok(stat)
    }
}