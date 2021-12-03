use std::process::{self, Stdio};
use mfek_ipc::{IPCInfo, module_available};
use test_env_log::test;

const KMD: &str = "metadata";
#[test]
fn metadata_available() {
    let (status, name) = module_available(KMD.into(), env!("CARGO_PKG_VERSION"));
    assert!(status.assert());
    assert!(process::Command::new(name).stderr(Stdio::null()).stdout(Stdio::null()).stdin(Stdio::null()).status().is_ok());
}

#[test]
fn info() {
    let ipcinfo = IPCInfo::new_disconnected();
    let s: String = ipcinfo.parent_exe.to_str().unwrap().to_string();
    assert!(s.contains("ipc.rlib"));
}
