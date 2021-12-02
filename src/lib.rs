pub static KMDBIN: &str = "ipc.rlib";

pub(crate) mod module;
pub(crate) mod info;
pub(crate) mod util;
pub mod helpers;
pub mod notifythread;

pub use util::InUfo; //trait
pub use module::{Available, name as module_name, available as module_available};
pub use info::IPCInfo;
