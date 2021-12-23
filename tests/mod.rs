mod helpers;

use mfek_ipc::{module, IPCInfo};
use std::process::{self, Stdio};
use test_log::test;

const KMD: &str = "metadata";
#[test]
fn module_available() {
    let (version, path) = module::available(KMD.into(), env!("CARGO_PKG_VERSION")).unwrap();
    assert!(version.matches());
    eprintln!("available {:?} {:?}", &version, &path);
    assert!(process::Command::new(path)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .status()
        .is_ok());
}

#[test]
fn ipcinfo() {
    let ipcinfo = IPCInfo::new_disconnected();
    let s: String = ipcinfo.parent_exe.to_str().unwrap().to_string();
    assert!(s.contains("ipc.rlib"));
}
