pub static KMDBIN: &str = "ipc.rlib";

pub mod module;
pub(crate) mod info;
pub(crate) mod util;
mod header;
pub mod helpers;
pub mod notifythread;

pub use header::{display as display_header, header};
pub use util::InUfo; //trait
pub use info::IPCInfo;
