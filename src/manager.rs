use users::{get_current_uid};
use std::{
    fs,
    path::{
        PathBuf
    }
};
use crate::{error::Result};
use crate::error::CGroupError;
use crate::cgroup::CGroup;

#[derive(Debug)]
pub struct Manager{
    path:  PathBuf
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
            },
            Err(err) => {
                Err(CGroupError::FileSystemFailure(err))
            }
        }
    }

    pub fn delete_child(&self, cgroup_name: &str) -> Result<()> {
        let mut path = PathBuf::from(&self.path);
        path.push(cgroup_name);
        return match fs::remove_dir(path) {
            Ok(()) => Ok(()),
            Err(err) => Err(CGroupError::FileSystemFailure(err))
        }
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
    use crate::manager::Manager;
    use crate::controller::ControllerType;

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
    }

    #[test]
    fn new_child() {
        let manager = Manager::default();
        let child = manager.new_child("cgv2");
        assert!(child.is_ok());
        dbg!(&child);
        let child = child.unwrap();
        let c_group = child.cgroup();

        let enables = vec![ControllerType::CPU, ControllerType::MEMORY];
        let disables = Some(vec![ControllerType::IO]);
        let result = c_group.set_subtree_control(enables, disables);
        dbg!(result);
        let result = c_group.subtree_control();
        dbg!(result);
        let result = c_group.controllers();
        dbg!(result);
        let result = c_group.cg_type();
        dbg!(result);
        let result = c_group.procs();
        dbg!(result);
        let result = manager.delete_child("cgv2");
        assert!(result.is_ok())
    }

}