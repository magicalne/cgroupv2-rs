use std::{
    str::FromStr,
    path::{
        Path,
        PathBuf
    }
};
use crate::{
    error::{
        Result,
        CGroupError
    },
    controller::ControllerType,
    util::{
        read_file_into_string,
        read_single_value,
        read_space_separated_values
    }
};
use crate::util::read_newline_separated_values;

pub struct CGroup<'a> {
    path: &'a Path
}

impl <'a> CGroup<'a> {
    pub fn new(path: &'a Path) -> Self {
        CGroup {
            path
        }
    }
    ///cgroup.controllers
    pub fn controllers(&self) -> Result<Vec<ControllerType>> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.controllers");
        let content = read_file_into_string(path.as_path())?;
        Ok(read_space_separated_values(content))
    }

    ///cgroup.subtree_control
    pub fn subtree_control(&self) -> Result<Vec<ControllerType>> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.subtree_control");
        let content = read_file_into_string(path.as_path())?;
        Ok(read_space_separated_values(content))
    }

    ///cgroup.type
    pub fn cg_type(&self) -> Result<CGroupType> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.type");
        let content = read_file_into_string(path.as_path())?;
        read_single_value(content)
    }

    ///cgruop.procs
    pub fn procs(&self) -> Result<Vec<i32>> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.procs");
        let content = read_file_into_string(path.as_path())?;
        Ok(read_newline_separated_values(content))
    }
}

#[derive(Debug)]
pub enum CGroupType {
    Domain,
    DomainThreaded,
    DomainInvalid,
    Threaded
}

impl FromStr for CGroupType {
    type Err = CGroupError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        return match s {
            "domain\n" => Ok(CGroupType::Domain),
            "domain threaded\n" => Ok(CGroupType::DomainThreaded),
            "domain invalid\n" => Ok(CGroupType::DomainInvalid),
            "threaded\n" => Ok(CGroupType::Threaded),
            _ => Err(CGroupError::UnknownField(String::from(s)))
        }
    }
}
