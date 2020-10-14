use std::{
    path::Path,
    str::FromStr,
};

use crate::{
    error::{
        CGroupError, Result,
    },
    FlatKeyedSetter,
    psi::CPUPressure,
    util::{
        read_flat_keyed_file, read_single_value, write_single_value,
    },
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
        read_flat_keyed_file(&self.path, filename)
    }

    pub fn weight(&self) -> Result<u16> {
        let filename = "cpu.weight";
        read_single_value(&self.path, filename)
    }

    pub fn set_weight(&self, w: u16) -> Result<()> {
        let filename = "cpu.weight";
        write_single_value(&self.path, filename, w)
    }

    pub fn weight_nice(&self) -> Result<i8> {
        let filename = "cpu.weight.nice";
        read_single_value(&self.path, filename)
    }

    pub fn set_weight_nice(&self, n: i8) -> Result<()> {
        let filename = "cpu.weight.nice";
        write_single_value(&self.path, filename, n)
    }

    pub fn max(&self) -> Result<CPUMax> {
        let filename = "cpu.max";
        read_single_value(&self.path, filename)
    }

    pub fn set_max(&self, max: u32, period: Option<u32>) -> Result<()> {
        let filename = "cpu.max";
        let max = CPUMax {
            max: CPUMaxMax::Val(max),
            period,
        };
        write_single_value(&self.path, filename, max)
    }

    pub fn pressure(&self) -> Result<CPUPressure> {
        let filename = "cpu.pressure";
        read_single_value(&self.path, filename)
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

#[derive(Debug, Eq, PartialEq)]
pub struct CPUMax {
    pub max: CPUMaxMax,
    pub period: Option<u32>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CPUMaxMax {
    Max,
    Val(u32),
}

impl ToString for CPUMaxMax {
    fn to_string(&self) -> String {
        match self {
            CPUMaxMax::Max => String::from("max"),
            CPUMaxMax::Val(max) => max.to_string()
        }
    }
}

impl FromStr for CPUMax {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<Self> {
        let mut kv = s.split_whitespace();
        let max = kv.next()
            .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
        let max = match max {
            "max" => CPUMaxMax::Max,
            _ => CPUMaxMax::Val(u32::from_str(max)
                .map_err(|_| CGroupError::UnknownFieldErr(s.to_string()))?)
        };
        let period = kv.next()
            .ok_or(CGroupError::UnknownFieldErr(s.to_string()))?;
        let period = Some(u32::from_str(period)
            .map_err(|_| CGroupError::UnknownFieldErr(s.to_string()))?);
        Ok(CPUMax {
            max,
            period,
        })
    }
}

impl ToString for CPUMax {
    fn to_string(&self) -> String {
        let max = self.max.to_string();
        return if let Some(period) = self.period {
            format!("{} {}", max, period.to_string())
        } else {
            max
        };
    }
}
