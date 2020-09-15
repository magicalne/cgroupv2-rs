use std::{
    fs,
    str::FromStr,
    io::Read,
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
    util::read_space_separated_values
};
use crate::util::read_single_value;

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
    pub fn controllers(&self) -> Result<Vec<Result<ControllerType>>> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.controllers");
        read_space_separated_values(path.as_path())
    }

    ///cgroup.subtree_control
    pub fn subtree_control(&self) -> Result<Vec<Result<ControllerType>>> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.subtree_control");
        read_space_separated_values(path.as_path())
    }

    ///cgroup.type
    pub fn cg_type(&self) -> Result<Result<CGroupType>> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.type");
        read_single_value(path.as_path())
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
            "domain" => Ok(CGroupType::Domain),
            "domain threaded" => Ok(CGroupType::DomainThreaded),
            "domain invalid" => Ok(CGroupType::DomainInvalid),
            "threaded" => Ok(CGroupType::Threaded),
            _ => Err(CGroupError::UnknownField(String::from(s)))
        }
    }
}

//Read cgroup interface files.
impl CGroupType {
    /// “domain” : A normal valid domain cgroup.
    /// “domain threaded” : A threaded domain cgroup which is serving as the root of a threaded subtree.
    /// “domain invalid” : A cgroup which is in an invalid state. It can’t be populated or have controllers enabled. It may be allowed to become a threaded cgroup.
    /// “threaded” : A threaded cgroup which is a member of a threaded subtree.
    fn parse(mut path: PathBuf) -> Option<CGroupType> {
        path.push("cgroup.type");
        if let Ok(mut file) = fs::File::open(path) {
            let mut vec = Vec::new();
            if let Ok(s) = file.read_to_end(&mut vec) {
                if s > 1 {
                    return match &vec[..s-1] {
                        b"domain" => Some(Self::Domain),
                        b"domain threade" => Some(Self::DomainThreaded),
                        b"domain invalid" => Some(Self::DomainInvalid),
                        b"Threaded" => Some(Self::Threaded),
                        _ => None
                    }
                }
            }
        }
        None
    }
}