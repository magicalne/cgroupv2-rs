use std::{
    fs::{
        self,
        OpenOptions,
    },
    io::Write,
    path::{
        Path,
        PathBuf,
    },
    str::FromStr,
};

use crate::{controller::ControllerType, io::IO, error::{
        CGroupError,
        Result,
    }, util::{
        read_file_into_string,
        read_newline_separated_values,
        read_space_separated_values,
    }};
use crate::cpu::Cpu;
use crate::memory::Memory;

pub struct CGroup<'a> {
    path: &'a Path
}

impl<'a> CGroup<'a> {
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
                               disables: Option<Vec<ControllerType>>,
    ) -> Result<()> {
        let mut line: String = enables
            .iter()
            .map(|e| {
                let mut s = e.to_string();
                s.insert_str(0, "+");
                return s;
            })
            .collect::<Vec<String>>()
            .join(" ");
        if let Some(disables) = disables {
            let _line = disables
                .iter()
                .map(|e| {
                    let mut s = e.to_string();
                    s.insert_str(0, "-");
                    return s;
                })
                .collect::<Vec<String>>()
                .join(" ");
            line.push(' ');
            line.push_str(&_line);
        };
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.subtree_control");
        fs::write(path.as_path(), line)
            .map_err(|e| CGroupError::FSErr(e.kind()))
    }

    ///cgroup.type
    pub fn cg_type(&self) -> Result<CGroupType> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.type");
        let content = read_file_into_string(path.as_path())?;
        CGroupType::from_str(&content)
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
            .map_err(|err| CGroupError::FSErr(err.kind()))?;
        match file.write(&pid.to_string().as_bytes()) {
            Ok(size) => {
                if size == 0 {
                    Err(CGroupError::WriteZeroByteErr)
                } else {
                    Ok(())
                }
            }
            Err(err) => {
                Err(CGroupError::FSErr(err.kind()))
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
            .map_err(|err| CGroupError::FSErr(err.kind()))?;
        match file.write(&tid.to_string().as_bytes()) {
            Ok(size) => {
                if size == 0 {
                    Err(CGroupError::WriteZeroByteErr)
                } else {
                    Ok(())
                }
            }
            Err(err) => {
                Err(CGroupError::FSErr(err.kind()))
            }
        }
    }

    ///cgroup.events
    pub fn events(&self) -> Result<CGroupEvent> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.events");
        let content = read_file_into_string(path.as_path())?;
        CGroupEvent::from_str(&content)
    }

    ///cgroup.max.descendants
    pub fn max_descendants(&self) -> Result<Max> {
        let filename = "cgroup.max.descendants";
        let mut path = PathBuf::from(&self.path);
        path.push(filename);
        let content = read_file_into_string(path.as_path())?;
        Max::from_str(&content)
    }

    ///cgroup.max.descendants
    pub fn set_max_descendants(&self, max: u32) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.max.descendants");
        fs::write(path.as_path(), max.to_string())
            .map_err(|e| CGroupError::FSErr(e.kind()))
    }

    ///cgroup.max.depth
    pub fn max_depth(&self) -> Result<Max> {
        let filename = "cgroup.max.depth";
        let mut path = PathBuf::from(&self.path);
        path.push(filename);
        let content = read_file_into_string(path.as_path())?;
        Max::from_str(&content)
    }

    ///cgroup.max.depth
    pub fn set_max_depth(&self, max: u32) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.max.depth");
        fs::write(path.as_path(), max.to_string())
            .map_err(|e| CGroupError::FSErr(e.kind()))
    }

    ///cgroup.stat
    pub fn stat(&self) -> Result<CGroupStat> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.stat");
        let content = read_file_into_string(path.as_path())?;
        CGroupStat::from_str(&content)
    }

    ///cgroup.freeze
    pub fn freeze(&self) -> Result<Freeze> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.freeze");
        let content = read_file_into_string(path.as_path())?;
        Freeze::from_str(&content)
    }

    ///cgroup.freeze
    pub fn set_freeze(&self) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push("cgroup.freeze");
        fs::write(path.as_path(), "1")
            .map_err(|e| CGroupError::FSErr(e.kind()))
    }

    ///cpu
    pub fn cpu(&self) -> Cpu {
        Cpu::new(self.path)
    }

    ///memory
    pub fn memory(&self) -> Memory {
        Memory::new(self.path)
    }

    ///io
    pub fn io(&self) -> IO {
        IO::new(self.path)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CGroupType {
    Domain,
    DomainThreaded,
    DomainInvalid,
    Threaded,
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
        };
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CGroupEvent {
    pub populated: bool,
    pub frozen: bool,
}

impl CGroupEvent {
    fn setter(&mut self, s: &str, val: bool) {
        match s {
            "populated" => self.populated = val,
            "frozen" => self.frozen = val,
            _ => {}
        }
    }
}

impl FromStr for CGroupEvent {
    type Err = CGroupError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut event = CGroupEvent {
            populated: false,
            frozen: false,
        };
        let mut splits = s.split('\n');
        for _ in 0..2 {
            let line = splits.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let mut kv = line.split_whitespace();
            let key = kv.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = kv.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = match val {
                "1" => true,
                _ => false
            };
            event.setter(key, val);
        }
        Ok(event)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Max {
    Max,
    Val(u32),
}

impl FromStr for Max {
    type Err = CGroupError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splits = s.split('\n');
        return match splits.next() {
            Some("max") => Ok(Max::Max),
            Some(max) => {
                let val = u32::from_str(max)
                    .map_err(|_| CGroupError::UnknownFieldErr(max.to_string()))?;
                Ok(Max::Val(val))
            }
            None => Err(CGroupError::EmptyFileErr)
        };
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CGroupStat {
    ///nr_descendants
    pub nr_descendants: u32,
    ///nr_dying_descendants
    pub nr_dying_descendants: u32,
}

impl CGroupStat {
    fn setter(&mut self, s: &str, val: u32) {
        match s {
            "nr_descendants" => self.nr_descendants = val,
            "nr_dying_descendants" => self.nr_dying_descendants = val,
            _ => {}
        }
    }
}

impl FromStr for CGroupStat {
    type Err = CGroupError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut stat = CGroupStat {
            nr_descendants: 0,
            nr_dying_descendants: 0,
        };
        let mut splits = s.split('\n');
        for _ in 0..2 {
            let line = splits.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let mut kv = line.split_whitespace();
            let key = kv.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = kv.next()
                .ok_or(CGroupError::UnknownFieldErr(String::from(s)))?;
            let val = u32::from_str(val)
                .map_err(|_| CGroupError::UnknownFieldErr(s.to_string()))?;
            stat.setter(key, val);
        }
        Ok(stat)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Freeze(pub bool);

impl FromStr for Freeze {
    type Err = CGroupError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splits = s.split('\n');
        if let Some(s) = splits.next() {
            return match s {
                "0" => Ok(Freeze(false)),
                "1" => Ok(Freeze(true)),
                _ => Err(CGroupError::UnknownFieldErr(s.to_string()))
            };
        }
        Err(CGroupError::UnknownFieldErr(s.to_string()))
    }
}