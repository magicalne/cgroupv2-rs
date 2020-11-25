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
        let filename = "io.stat";
        read_nested_keyed_file(&self.path, filename)
    }

    ///A read-write nested-keyed file with exists only on the root cgroup.
    pub fn cost_qos(&self) -> Result<HashMap<DeviceNumber, CostQos>> {
        let filename = "cost.qos";
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

#[derive(Debug)]
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
        let mut splits = s.split_whitespace();
        while let Some(next) = splits.next() {
            let mut kv = next.split('=');
            let key = kv.next().ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = kv.next().ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            stat.set(key, val)?;
        }
        Ok(stat)
    }
}

/// When “ctrl” is “auto”, the parameters are controlled by the kernel and may change automatically.
/// Setting “ctrl” to “user” or setting any of the percentile and latency parameters puts it into “user” mode
/// and disables the automatic changes. The automatic mode can be restored by setting “ctrl” to “auto”.
#[derive(Debug)]
pub enum Ctrl {
    Auto,
    User
}

impl FromStr for Ctrl {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<Self> {
        let ctrl = match s {
            "auto" => Self::Auto,
            "user" => Self::User,
            _ => {
                return Err(CGroupError::UnknownFieldErr(s.to_string()))
            }
        };
        Ok(ctrl)
    }
}
#[derive(Debug)]
pub struct CostQos {
    pub enable: u8, //Weight-based control enable
    pub ctrl: Ctrl, 
    pub rpct: f32, //Read latency percentile [0, 100]
    pub rlat: u32, //Read latency threshold
    pub wpct: f32, //Write latency percentile [0, 100]
    pub wlat: u32, //Write latency threshold
    pub min: u16, //Minimum scaling percentage [1, 10000]
    pub max: u16, //Maximum scaling percentage [1, 10000]
}

impl CostQos {
    fn set(&mut self, k: &str, v: &str) -> Result<()> {
        match k {
            "enabe" => self.enable = u8::from_str(v).map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?,
            "ctrl" => self.ctrl= Ctrl::from_str(v).map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?,
            "rpct" => self.rpct= f32::from_str(v).map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?,
            "rlat" => self.rlat = u32::from_str(v).map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?,
            "wpct" => self.wpct= f32::from_str(v).map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?,
            "wlat" => self.wlat= u32::from_str(v).map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?,
            "min" => self.min = u16::from_str(v).map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?,
            "max" => self.max = u16::from_str(v).map_err(|_| CGroupError::UnknownFieldErr(v.to_string()))?,
            _ => {
                return Err(CGroupError::UnknownFieldErr(v.to_string()))
            }
        }
        Ok(())
    }
}

impl FromStr for CostQos {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<Self> {
        let mut cost_qos = CostQos {
            enable: 0,
            ctrl: Ctrl::Auto,
            rpct: 0.0,
            rlat: 0,
            wpct: 0.0,
            wlat: 0,
            min: 0,
            max: 0,
        };
        let mut splits = s.split_whitespace();
        while let Some(next) = splits.next() {
            let mut kv = next.split('=');
            let key = kv.next().ok_or(CGroupError::UnknownFieldErr(s.to_string()))?;
            let val = kv.next().ok_or(CGroupError::UnknownFieldErr(s.to_string()))?;
            cost_qos.set(key, val)?;
        }
        Ok(cost_qos)
    }
}