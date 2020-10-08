use std::{
    path::Path,
};

use crate::{
    error::{
        Result,
    },
    util::{
        read_single_value
    },
    FlatKeyedSetter
};
use crate::util::{write_single_value, read_flat_keyed_file};

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
        read_flat_keyed_file(&self.path, filename)
    }

    pub fn weight(&self) -> Result<u16> {
        let filename = "cpu.weight";
        read_single_value(&self.path, filename)
    }

    pub fn set_weight(&self, weight: u16) -> Result<()> {
        let filename = "cpu.weight";
        write_single_value(&self.path, filename, weight)
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

impl FlatKeyedSetter<u64> for Stat {
    fn new() -> Self {
        Stat {
            usage_usec: 0,
            user_usec: 0,
            system_usec: 0,
            nr_periods: 0,
            nr_throttled: 0,
            throttled_usec: 0,
        }
    }

    fn set(&mut self, s: &str, val: u64) {
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
