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
use crate::common::Max;
use std::collections::HashMap;
use crate::util::{read_flat_keyed_file_map, read_value};
use crate::psi::MemoryPressure;

#[derive(Debug, Eq, PartialEq)]
pub struct Memory<'a> {
    path: &'a Path
}

impl<'a> Memory<'a> {
    pub fn new(path: &'a Path) -> Self {
        Memory {
            path
        }
    }

    pub fn current(&self) -> Result<u64> {
        let filename = "memory.current";
        read_single_value(&self.path, filename)
    }

    pub fn min(&self) -> Result<u64> {
        let filename = "memory.min";
        read_single_value(&self.path, filename)
    }

    pub fn set_min(&self, min: u64) -> Result<()> {
        let filename = "memory.min";
        write_single_value(&self.path, filename, min)
    }

    pub fn low(&self) -> Result<u64> {
        let filename = "memory.low";
        read_single_value(&self.path, filename)
    }

    pub fn set_low(&self, low: u64) -> Result<()> {
        let filename = "memory.low";
        write_single_value(&self.path, filename, low)
    }

    pub fn high(&self) -> Result<u64> {
        let filename = "memory.high";
        read_single_value(&self.path, filename)
    }

    pub fn set_high(&self, high: u64) -> Result<()> {
        let filename = "memory.high";
        write_single_value(&self.path, filename, high)
    }

    pub fn max(&self) -> Result<Max> {
        let filename = "memory.max";
        read_single_value(&self.path, filename)
    }

    pub fn set_max(&self, max: u64) -> Result<()> {
        let filename = "memory.max";
        write_single_value(&self.path, filename, max)
    }

    pub fn oom_group(&self) -> Result<u8> {
        let filename = "memory.oom.group";
        read_single_value(&self.path, filename)
    }

    pub fn set_oom_group(&self, i: u8) -> Result<()> {
        let filename = "memory.oom.group";
        write_single_value(&self.path, filename, i)
    }

    pub fn events(&self) -> Result<Event> {
        let filename = "memory.events";
        read_flat_keyed_file(&self.path, filename)
    }

    pub fn events_local(&self) -> Result<Event> {
        let filename = "memory.events.local";
        read_flat_keyed_file(&self.path, filename)
    }

    pub fn stat(&self) -> Result<HashMap<String, u64>> {
        let filename = "memory.stat";
        read_flat_keyed_file_map(&self.path, filename)
    }

    pub fn swap_current(&self) -> Result<u64> {
        let filename = "memory.swap.current";
        read_single_value(&self.path, filename)
    }

    pub fn swap_high(&self) -> Result<Max> {
        let filename = "memory.swap.high";
        read_single_value(&self.path, filename)
    }

    pub fn set_swap_high(&self, max: u32) -> Result<()> {
        let filename = "memory.swap.high";
        write_single_value(&self.path, filename, max)
    }

    pub fn swap_max(&self) -> Result<Max> {
        let filename = "memory.swap.max";
        read_single_value(&self.path, filename)
    }

    pub fn set_swap_max(&self, max: u32) -> Result<()> {
        let filename = "memory.swap.max";
        write_single_value(&self.path, filename, max)
    }

    pub fn swap_events(&self) -> Result<SwapEvent> {
        let filename = "memory.swap.events";
        read_flat_keyed_file(&self.path, filename)
    }

    pub fn pressure(&self) -> Result<MemoryPressure> {
        let filename = "memory.pressure";
        read_value(&self.path, filename)
    }

}

#[derive(Debug, Eq, PartialEq)]
pub struct Event {
    pub low: u32,
    pub high: u32,
    pub max: u32,
    pub oom: u32,
    pub oom_kill: u32
}

impl FlatKeyedSetter<u32> for Event {
    fn new() -> Self {
        Event {
            low: 0,
            high: 0,
            max: 0,
            oom: 0,
            oom_kill: 0
        }
    }

    fn set(&mut self, s: &str, val: u32) {
        match s {
            "low" => self.low = val,
            "high" => self.high = val,
            "max" => self.max = val,
            "oom" => self.oom = val,
            "oom_kill" => self.oom_kill = val,
            _ => {}
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SwapEvent {
    pub high: u32,
    pub max: u32,
    pub fail: u32
}

impl FlatKeyedSetter<u32> for SwapEvent {
    fn new() -> Self {
        SwapEvent {
            high: 0,
            max: 0,
            fail: 0
        }
    }

    fn set(&mut self, s: &str, val: u32) {
        match s {
            "high" => self.high = val,
            "max" => self.max = val,
            "fail" => self.fail = val,
            _ => {}
        }
    }
}