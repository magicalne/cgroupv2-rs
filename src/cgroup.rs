use std::{
    fs::{
        self,
        OpenOptions
    },
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
        read_space_separated_values,
        read_newline_separated_values
    }
};
use std::io::Write;

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

    pub fn set_subtree_control(&self,
                               enables: Vec<ControllerType>,
                               disables: Option<Vec<ControllerType>>
    ) -> Result<()> {
        let mut line: String = enables
            .iter()
            .map(|e| {
                let mut s = e.to_string();
                s.insert_str(0, "+");
                return s
            })
            .collect::<Vec<String>>()
            .join(" ");
        if let Some(disables) = disables {
            let _line = disables
                .iter()
                .map(|e| {
                    let mut s = e.to_string();
                    s.insert_str(0, "-");
                    return s
                })
                .collect::<Vec<String>>()
                .join(" ");
            line.push(' ');
            line.push_str(&_line);
        };
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.subtree_control");
        fs::write(path.as_path(), line)
            .map_err(|e| CGroupError::FSErr(e))
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

    ///cgroup.procs
    pub fn add_pid(&self, pid: u32) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.procs");
        let mut file = OpenOptions::new()
            .append(true)
            .open(&path)
            .map_err(|err| CGroupError::FSErr(err))?;
        match file.write(&pid.to_string().as_bytes()) {
            Ok(size) => {
                if size == 0 {
                    Err(CGroupError::WriteZeroByteErr)
                } else {
                    Ok(())
                }
             },
            Err(err) => {
                Err(CGroupError::FSErr(err))
            }
        }
    }

    ///cgroup.threads
    pub fn threads(&self) -> Result<Vec<i32>> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.threads");
        let content = read_file_into_string(path.as_path())?;
        Ok(read_newline_separated_values(content))
    }

    pub fn add_tid(&self, tid: u32) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.threads");
        let mut file = OpenOptions::new()
            .append(true)
            .open(&path)
            .map_err(|err| CGroupError::FSErr(err))?;
        match file.write(&tid.to_string().as_bytes()) {
            Ok(size) => {
                if size == 0 {
                    Err(CGroupError::WriteZeroByteErr)
                } else {
                    Ok(())
                }
            },
            Err(err) => {
                Err(CGroupError::FSErr(err))
            }
        }
    }

    ///cgroup.events
    pub fn events(&self) -> Result<Vec<CGroupEvent>> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.events");
        let content = read_file_into_string(path.as_path())?;
        Ok(read_newline_separated_values(content))
    }

    ///cgroup.max.descendants
    pub fn max_descendants(&self) -> Result<Max> {
        let filename = "cgroup.max.descendants";
        let mut path = PathBuf::from(&self.path);
        path.push(filename);
        let content = read_file_into_string(path.as_path())?;
        read_single_value(content)
    }

    ///cgroup.max.descendants
    pub fn set_max_descendants(&self, max: u32) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.max.descendants");
        fs::write(path.as_path(), max.to_string())
            .map_err(|e| CGroupError::FSErr(e))
    }

    ///cgroup.max.depth
    pub fn max_depth(&self) -> Result<Max> {
        let filename = "cgroup.max.depth";
        let mut path = PathBuf::from(&self.path);
        path.push(filename);
        let content = read_file_into_string(path.as_path())?;
        read_single_value(content)
    }

    ///cgroup.max.depth
    pub fn set_max_depth(&self, max: u32) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.max.depth");
        fs::write(path.as_path(), max.to_string())
            .map_err(|e| CGroupError::FSErr(e))
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
            _ => Err(CGroupError::UnknownFieldErr(String::from(s)))
        }
    }
}

#[derive(Debug)]
pub enum CGroupEvent {
    Populated(bool),
    Frozen(bool)
}

impl FromStr for CGroupEvent {
    type Err = CGroupError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splits = s.split_whitespace();
        if let Some(key) = splits.next() {
            if let Some(val) = splits.next() {
                let val = match val {
                    "1" => true,
                    _ => false
                };
                return match key {
                    "populated" => Ok(CGroupEvent::Populated(val)),
                    "frozen" => Ok(CGroupEvent::Frozen(val)),
                    _ => Err(CGroupError::UnknownFieldErr(String::from(s)))
                }
            }
        }
        return Err(CGroupError::UnknownFieldErr(String::from(s)))
    }
}

#[derive(Debug)]
pub enum Max {
    Max,
    Val(u32)
}

impl FromStr for Max {
    type Err = CGroupError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splits = s.split('\n');
        return match splits.next() {
            Some("max") => Ok(Max::Max),
            Some(max) => {
                let val = u32::from_str(max)
                    .map_err(|err| CGroupError::UnknownFieldErr(max.to_string()))?;
                Ok(Max::Val(val))
            },
            None => Err(CGroupError::EmptyFileErr)
        }
    }
}