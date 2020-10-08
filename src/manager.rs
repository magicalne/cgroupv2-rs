use std::{
    fs,
    path::PathBuf,
};

use users::get_current_uid;

use crate::{
    cgroup::CGroup,
    error::{
        CGroupError,
        Result,
    },
};

#[derive(Debug)]
pub struct Manager {
    path: PathBuf
}

impl Manager {
    pub fn cgroup(&self) -> CGroup {
        CGroup::new(self.path.as_path())
    }

    pub fn new_child(&self, cgroup_name: &str) -> Result<Manager> {
        let mut path = PathBuf::from(&self.path);
        path.push(cgroup_name);
        match fs::create_dir(&path) {
            Ok(_) => {
                Ok(Manager {
                    path
                })
            }
            Err(err) => {
                Err(CGroupError::FSErr(err.kind()))
            }
        }
    }

    pub fn delete_child(&self, cgroup_name: &str) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push(cgroup_name);
        return match fs::remove_dir(path) {
            Ok(()) => Ok(()),
            Err(err) => Err(CGroupError::FSErr(err.kind()))
        };
    }
}

const DEFAULT_MOUNT_POINT: &str = "/sys/fs/cgroup/";

impl Default for Manager {
    fn default() -> Self {
        let mount_point = DEFAULT_MOUNT_POINT;
        let path = PathBuf::from(get_delegate_path(mount_point));
        Manager {
            path
        }
    }
}

fn get_delegate_path(mount_point: &str) -> String {
    let uid = get_current_uid();
    let delegate_path = format!("{}/user.slice/user-{}.slice/user@{}.service/",
                                mount_point, uid, uid);
    delegate_path
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use crate::cgroup::{CGroupEvent, CGroupStat, CGroupType, Freeze, Max};
    use crate::controller::ControllerType;
    use crate::cpu::{Stat, CPUMax, CPUMaxMax};
    use crate::manager::Manager;

    #[test]
    fn enabled_controllers() {
        let manager = Manager::default();
        let c_group = manager.cgroup();
        let result = c_group.controllers();
        dbg!(&manager.path);
        assert!(result.is_ok());
    }

    #[test]
    fn enabled_subtree_control() {
        let manager = Manager::default();
        let c_group = manager.cgroup();
        let result = c_group.subtree_control();
        assert!(result.is_ok());
        let result = c_group.set_subtree_control(ControllerType::all(), None);
        dbg!(result);
    }

    #[test]
    fn new_child() {
        let manager = Manager::default();
        let child = manager.new_child("cgv2");
        assert!(child.is_ok());
        let child = child.unwrap();
        let c_group = child.cgroup();

        let enables = vec![ControllerType::MEMORY, ControllerType::PIDS];
        let disables = Some(vec![ControllerType::IO]);
        let result = c_group.set_subtree_control(enables, disables);
        assert!(result.is_ok());
        let result = c_group.subtree_control();
        assert_eq!(result, Ok(vec![ControllerType::MEMORY, ControllerType::PIDS]));
        let result = c_group.controllers();
        assert_eq!(result, Ok(vec![ControllerType::MEMORY, ControllerType::PIDS]));
        let result = c_group.cg_type();
        // assert_eq!(result, Ok::<Result<Vec<CGroupType>, CGroupError>>(CGroupType::Domain));
        assert_eq!(result, Ok(CGroupType::Domain));
        // let pid = std::process::id();
        let result = c_group.procs();
        assert_eq!(result, Ok(vec![]));
        let result = c_group.threads();
        assert_eq!(result, Ok(vec![]));
        let result = c_group.events();
        assert_eq!(result, Ok(CGroupEvent { populated: false, frozen: false }));
        let result = c_group.max_descendants();
        assert_eq!(result, Ok(Max::Max));
        c_group.set_max_descendants(15);
        let result = c_group.max_descendants();
        assert_eq!(result, Ok(Max::Val(15)));
        let result = c_group.max_depth();
        assert_eq!(result, Ok(Max::Max));
        let result = c_group.set_max_depth(16);
        assert!(result.is_ok());
        let result = c_group.max_depth();
        assert_eq!(result, Ok(Max::Val(16)));
        let result = c_group.stat();
        assert_eq!(result, Ok(CGroupStat { nr_descendants: 0, nr_dying_descendants: 0 }));
        let result = c_group.freeze();
        assert_eq!(result, Ok(Freeze(false)));
        let result = c_group.set_freeze();
        assert!(result.is_ok());
        let result = c_group.freeze();
        assert_eq!(result, Ok(Freeze(true)));
        let result = manager.delete_child("cgv2");
        assert!(result.is_ok())
    }

    #[test]
    fn cpu_test() {
        let manager = Manager::default();
        let cgroup_name = "mycgv2";
        let result = manager.new_child(cgroup_name);
        let child = result.unwrap();
        let c_group = child.cgroup();
        let cpu = c_group.cpu();
        let stat = cpu.stat();
        let expect = Ok(Stat {
            usage_usec: 0,
            user_usec: 0,
            system_usec: 0,
            nr_periods: 0,
            nr_throttled: 0,
            throttled_usec: 0});
        assert_eq!(stat, expect);

        let weight = cpu.set_weight(20);
        assert_eq!(weight, Ok(()));
        let weight = cpu.weight();
        assert_eq!(weight, Ok(20));

        let weight_nice = cpu.set_weight_nice(-1);
        assert_eq!(weight_nice, Ok(()));
        let weight_nice = cpu.weight_nice();
        assert_eq!(weight_nice, Ok(-1));

        let max = cpu.max();
        assert_eq!(max, Ok(CPUMax{max: CPUMaxMax::Max, period: Some(100000)}));
        let set_max = cpu.set_max(u32::max_value(), None);
        assert_eq!(set_max, Ok(()));
        let max = cpu.max();
        assert_eq!(max, Ok(CPUMax{max: CPUMaxMax::Val(u32::max_value()), period: Some(100000)}));
        manager.delete_child(cgroup_name);
    }
}