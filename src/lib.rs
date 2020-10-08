mod manager;
pub mod error;
pub mod util;
pub mod controller;
pub mod cgroup;
pub mod cpu;
pub mod psi;

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
/// can have domain controllers enabled in their "cgroup.subtree_control" files.
/// In my case, the root privilege is still necessary if you want to enable "cpuset" in
/// cgroup.subtree_control.
/// TODO

pub trait FlatKeyedSetter<T> {
    fn new() -> Self;
    fn set(&mut self, s: &str, val: T);
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
