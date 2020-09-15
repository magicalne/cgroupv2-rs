mod manager;
pub mod error;
pub mod util;
pub mod controller;
pub mod cgroup;

use std::{
    fs,
    path::{
        PathBuf
    }
};
use std::io::Read;
use std::convert::TryInto;

/// This is a native rust lib for (cgroup V2)[https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html].
/// The default base path of cgroup should be **/sys/fs/cgroup**. Or you can mount a new fs if you
/// like to. This lib use rootless privilege. And it involves some help of systemd. You should have
/// your cgroup v2 initialized by systemd already.
/// ```console
/// # mkdir -p /etc/systemd/system/user@.service.d
/// # cat > /etc/systemd/system/user@.service.d/delegate.conf << EOF
/// [Service]
/// Delegate=cpu Cpuset io memory pids
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

//parse cgroup.procs TODO
fn parse_cgroup_procs(mut path: PathBuf) -> Option<Vec<u32>> {
    path.push("cgroup.procs");
    if let Ok(mut file) = fs::File::open(path) {
        let mut buf = Vec::new();
        if let Ok(s) = file.read_to_end(&mut buf) {
            dbg!(String::from_utf8_lossy(&buf));
            if s > 1 {
                let pids = buf.split(|e| *e == b'\n')
                    .map(|arr| {
                        return u32::from_be_bytes(arr.try_into().unwrap())
                    })
                    .collect::<Vec<u32>>();
                dbg!(&pids);
                // if !pids.is_empty() {
                //     Some(pids)
                // }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
