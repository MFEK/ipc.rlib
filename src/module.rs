use log;

use std::{env, fs};
use std::ffi::OsString;
use std::path::PathBuf;

use crate::module;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Available {
    Yes,
    /// FIXME: Unimplemented. I should add module version checking.
    Degraded,
    No,
}

pub fn name(module: &str) -> Vec<String> {
    #[cfg(target_family = "windows")]
    let module = vec![
        format!("MFEK{}.exe", module),
        format!("mfek-{}.exe", module),
    ];
    #[cfg(not(target_family = "windows"))]
    let module = vec![format!("MFEK{}", module), format!("mfek-{}", module)];

    module
}

impl Available {
    pub fn assert(&self) -> bool {
        *self == Available::Yes
    }
}

pub fn available(module: &str) -> (Available, String) {
    let mut ret = Available::No;
    let modules = module::name(module);
    for mn in modules.iter() {
        match env::var_os("PATH") {
            Some(paths) => {
                for path in env::split_paths(&paths) {
                    let pb: PathBuf = [path.as_os_str(), &OsString::from(mn.clone())]
                        .iter()
                        .collect();
                    log::debug!("Checking {:?} for {:?}", &pb, &mn);
                    let omd = fs::metadata(&pb);
                    match omd {
                        Ok(md) => {
                            if md.is_file() {
                                log::debug!("Got metadata: {:?}", &md);
                                #[cfg(target_family = "unix")]
                                {
                                    use std::os::unix::fs::PermissionsExt;
                                    if md.permissions().mode() & 0o111 != 0 {
                                        ret = Available::Yes;
                                    }
                                }
                                #[cfg(not(target_family = "unix"))]
                                {
                                    ret = Available::Yes;
                                }
                                log::info!("{:?} found", &pb);

                                return (ret, mn.clone());
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            None => {}
        }
    }
    log::error!(
        "Module MFEK{} is not available. MFEK is modular software; it will still run but some \
        features will not be available. For the best experience, please install all available \
        MFEK modules into your PATH.",
        module
    );
    (ret, String::new())
}
