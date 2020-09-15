use std::str::FromStr;
use crate::error::CGroupError;

#[derive(Debug)]
pub enum ControllerType {
    CPUSET, CPU, IO, MEMORY, PIDS
}

impl FromStr for ControllerType {
    type Err = CGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "cpuset" => Ok(ControllerType::CPUSET),
            "cpu" => Ok(ControllerType::CPU),
            "io" => Ok(ControllerType::IO),
            "memory" => Ok(ControllerType::MEMORY),
            "pids" => Ok(ControllerType::PIDS),
            _ => Err(CGroupError::UnknownField(String::from(s)))
        }
    }
}