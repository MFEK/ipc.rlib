use test_log::test;
use mfek_ipc::{display_header, header as ipc_header};
use std::env;

#[test]
fn header() {
    // ignore user colorize options just for this test
    env::remove_var("NO_COLOR");
    env::set_var("CLICOLOR_FORCE", "1");
    let header = ipc_header("ipc");
    display_header("ipc");
    assert_eq!(header, include_bytes!("../test_data/header_ansi.txt"));
}
