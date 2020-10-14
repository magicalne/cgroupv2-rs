use std::str::FromStr;
use crate::error::{
    Result,
    CGroupError
};

#[derive(Debug, PartialEq)]
pub struct CPUPressure {
    pub(crate) some: PSIMetric,
}

#[derive(Debug, PartialEq)]
pub struct PSIMetric {
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

impl FromStr for CPUPressure {
    type Err = CGroupError;

    //some avg10=0.00 avg60=0.00 avg300=0.00 total=0
    fn from_str(s: &str) -> Result<Self> {
        let mut splits = s.split_whitespace();
        let _ = splits.next();
        let mut metric = PSIMetric {
            avg10: 0.0,
            avg60: 0.0,
            avg300: 0.0,
            total: 0
        };
        for _ in 0..4 {
            let split = splits.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let mut kv = split.split('=');
            let key = kv.next().ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = kv.next().ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            metric.set(key, val)?;
        }
        return Ok(CPUPressure {
            some: metric
        })
    }
}