use std::str::FromStr;
use crate::error::CGroupError;

#[derive(Debug)]
pub enum ControllerType {
    CPUSET, CPU, IO, MEMORY, PIDS
}

impl ControllerType {
    pub fn all() -> Vec<ControllerType> {
        vec![Self::CPUSET, Self::CPU, Self::IO, Self::MEMORY, Self::PIDS]
    }
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
            _ => Err(CGroupError::UnknownFieldErr(String::from(s)))
        }
    }
}

impl ToString for ControllerType {
    fn to_string(&self) -> String {
        let s = match self {
            ControllerType::CPUSET => "cpuset",
            ControllerType::CPU => "cpu",
            ControllerType::IO => "io",
            ControllerType::MEMORY => "memory",
            ControllerType::PIDS => "pids",
        };
        s.to_string()
    }
}