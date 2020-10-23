use std::str::FromStr;
use crate::error::{
    Result,
    CGroupError
};

#[derive(Debug, PartialEq)]
pub struct CPUPressure {
    pub(crate) some: PSIMetric,
}

impl FromStr for CPUPressure {
    type Err = CGroupError;

    //some avg10=0.00 avg60=0.00 avg300=0.00 total=0
    fn from_str(s: &str) -> Result<Self> {
        let metric = PSIMetric::from_str(s)?;
        return Ok(CPUPressure {
            some: metric
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct MemoryPressure {
    pub some: PSIMetric,
    pub full: PSIMetric
}

impl FromStr for MemoryPressure {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.split('\n');
        let mut pressure = MemoryPressure {
            some: PSIMetric {
                key: "".to_string(),
                avg10: 0.0,
                avg60: 0.0,
                avg300: 0.0,
                total: 0
            },
            full: PSIMetric {
                key: "".to_string(),
                avg10: 0.0,
                avg60: 0.0,
                avg300: 0.0,
                total: 0
            }
        };
        while let Some(next) = lines.next() {
            if !next.is_empty() {
                let metric = PSIMetric::from_str(next)?;
                match metric.key.as_ref() {
                    "some" => pressure.some = metric,
                    "full" => pressure.full = metric,
                    _ => {}
                }
            }
        }
        Ok(pressure)
    }
}

#[derive(Debug, PartialEq)]
pub struct PSIMetric {
    pub key: String,
    pub avg10: f32,
    pub avg60: f32,
    pub avg300: f32,
    pub total: u64
}

impl PSIMetric {

    pub fn set(&mut self, key: &str, val: &str) -> Result<()>{
        match key {
            "avg10" => {
                let val = f32::from_str(val)
                    .map_err(|_| CGroupError::UnknownFieldErr(val.to_string()))?;
                self.avg10 = val;
            },
            "avg60" => {
                let val = f32::from_str(val)
                    .map_err(|_| CGroupError::UnknownFieldErr(val.to_string()))?;
                self.avg60 = val;
            },
            "avg300" => {
                let val = f32::from_str(val)
                    .map_err(|_| CGroupError::UnknownFieldErr(val.to_string()))?;
                self.avg300 = val;
            },
            "total" => {
                let val = u64::from_str(val)
                    .map_err(|_| CGroupError::UnknownFieldErr(val.to_string()))?;
                self.total = val;
            },
            _ => {}
        }
        return Ok(())
    }
}

impl FromStr for PSIMetric {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<Self> {
        let mut splits = s.split_whitespace();
        let key = splits.next()
            .ok_or(CGroupError::UnknownFieldErr(s.to_string()))?;
        let mut metric = PSIMetric {
            key: key.to_string(),
            avg10: 0.0,
            avg60: 0.0,
            avg300: 0.0,
            total: 0
        };
        while let Some(next) = splits.next() {
            let mut kv = next.split('=');
            let key = kv.next().ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = kv.next().ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            metric.set(key, val)?;
        }
        Ok(metric)
    }
}