use std::str::FromStr;
use crate::error::{
    CGroupError,
    Result
};

#[derive(Debug, Eq, PartialEq)]
pub enum Max {
    Max,
    Val(u32),
}

impl ToString for Max {
    fn to_string(&self) -> String {
        match self {
            Max::Max => String::from("max"),
            Max::Val(max) => max.to_string()
        }
    }
}

impl FromStr for Max {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<Self> {
        let max = match s {
            "max" => Max::Max,
            _ => Max::Val(u32::from_str(s)
                .map_err(|_| CGroupError::UnknownFieldErr(s.to_string()))?)
        };
        Ok(max)
    }
}