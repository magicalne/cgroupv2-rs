use std::{
    fs,
    io,
    path::{
        Path,
        PathBuf
    }
};
use users::{get_current_uid};
use std::io::Read;
use std::fmt::{Debug, Formatter};

/// This is a native rust lib for (cgroup V2)[https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html].
/// The default base path of cgroup should be **/sys/fs/cgroup**. Or you can mount a new fs if you
/// like to. This lib use rootless privilege. And it involves some help of systemd. You should have
/// your cgroup v2 initialized by systemd already.
/// ```console
/// # mkdir -p /etc/systemd/system/user@.service.d
/// # cat > /etc/systemd/system/user@.service.d/delegate.conf << EOF
/// [Service]
/// Delegate=cpu cpuset io memory pids
/// EOF
/// # systemctl daemon-reload
/// ```
/// The path on my Fedora32 looks like "/sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service".
/// So this lib can operate under the path without root privilege.
/// Constrains:
/// Non-root cgroups can distribute domain resources to their children only when they don’t have any
/// processes of their own. In other words, only domain cgroups which don’t contain any processes
/// can have domain controllers enabled in their “cgroup.subtree_control” files.
/// TODO
const DEFAULT_MOUNT_POINT: &str = "/sys/fs/cgroup/";
trait CgroupManager {

}

struct FS {
    mount_point: String,
    cgroup_path:  PathBuf
}

impl FS {
    fn create(&self, name: &str) -> Option<CGroup> {
        let mut cgroup_path = self.cgroup_path.clone();
        cgroup_path.push(name);
        if fs::create_dir_all(&cgroup_path).is_ok() {
            CGroup::new(cgroup_path.to_str().unwrap())
        } else {
            None
        }
    }

    fn delete(&self, name: &str) -> io::Result<()> {
        let mut cgroup_path = self.cgroup_path.clone();
        cgroup_path.push(name);
        fs::remove_dir(cgroup_path)
    }
}

impl Default for FS {
    fn default() -> Self {
        let cgroup_path = get_delegate_path(DEFAULT_MOUNT_POINT);
        FS {
            mount_point: DEFAULT_MOUNT_POINT.to_string(),
            cgroup_path
        }
    }
}

#[derive(Debug)]
struct CGroup {
    path: PathBuf,
    cgroup_type: Option<CGroupType>
}

impl CGroup {
    fn new(path: &str) -> Option<Self> {
        let path = PathBuf::from(path);
        if path.is_dir() {
            let cgroup_type = CGroupType::parse(path.clone());
            Some(CGroup {
                path,
                cgroup_type
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum CGroupType {
    Domain,
    DomainThreaded,
    DomainInvalid,
    Threaded
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


fn get_delegate_path(mount_point: &str) -> PathBuf {
    let uid = get_current_uid();
    let delegate_path = format!("user.slice/user-{}.slice/user@{}.service/", uid, uid);
    let mut cgroup_path = PathBuf::new();
    cgroup_path.push(mount_point);
    cgroup_path.push(delegate_path.as_str());
    cgroup_path
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn fs_create_delete_test() {
        let cgroup_name = "mycgv2";
        let fs = FS::default();
        let cgroup = fs.create(cgroup_name);
        dbg!(&cgroup);
        assert!(cgroup.is_some());
        let result = fs.delete(cgroup_name);
        assert!(result.is_ok());
    }

    #[test]
    fn get_delegate_path_test() {
        let path_buf = get_delegate_path(DEFAULT_MOUNT_POINT);
        let target = "/sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service/";
        assert_eq!(path_buf.to_str(), Some(target));
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
