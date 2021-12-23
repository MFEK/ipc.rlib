use mfek_ipc::IPCInfo;
use mfek_ipc::helpers::metadata::*;
use test_log::{self, test};
#[test]
fn test_arbitrary() {
    use std::{env, path};
    eprintln!("{:?}", env::current_dir());
    let mut info = IPCInfo::new_disconnected();
    info.font = Some(path::PathBuf::from("test_data/FRBAmericanCursive-SOURCE.ufo/").into());
    let arb = arbitrary(&info, &["openTypeNameLicense", "openTypeNameVersion"]).unwrap();
    assert!(arb["openTypeNameLicense"].contains("GNU"));
    eprintln!("{}", arb["openTypeNameLicense"]);
}
#[test]
fn test_asc_desc() {
    let mut info = IPCInfo::new_disconnected();
    info.font = Some(("test_data/FRBAmericanCursive-SOURCE.ufo/").into());
    let a_d = ascender_descender(&info).unwrap();
    assert_eq!(a_d.0, 650.0);
    assert_eq!(a_d.1, -350.0);
}
