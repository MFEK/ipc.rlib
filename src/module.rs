use log;

use std::env::{self, current_exe};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::str as stdstr;

#[derive(Debug, Clone, PartialEq)]
pub enum Version<'caller> {
    UpToDate(&'caller str),
    OutOfDate(Option<String>),
}

impl Version<'_> {
    pub fn matches(&self) -> bool {
        match self {
            Self::UpToDate(_) => true,
            Self::OutOfDate(_) => false,
        }
    }
}

pub fn binaries(module: &str) -> Vec<String> {
    #[cfg(target_family = "windows")]
    let mut binaries;
    #[cfg(not(target_family = "windows"))]
    let binaries;
    binaries = vec![format!("MFEK{}", module), format!("mfek-{}", module)];
    #[cfg(target_family = "windows")]
    binaries.extend([format!("MFEK{}.exe", module), format!("mfek-{}.exe", module)]);

    binaries
}

pub fn available<'caller>(module: &str, version: &'caller str) -> Result<(Version<'caller>, PathBuf), ()> {
    let modules = binaries(module);
    let bindir = current_exe().map(|pb| vec![pb.parent().unwrap().to_owned()]).unwrap_or(vec![]);
    let paths = match env::var_os("PATH") {
        Some(paths) => {
            env::split_paths(&paths).chain(bindir).collect()
        }
        None => bindir,
    };

    for path in paths {
        for mn in &modules {
            let pb: PathBuf = [path.as_os_str(), &OsString::from(mn.clone())].iter().collect();
            log::debug!("Checking {:?} for {:?}", &pb, &mn);
            let omd = fs::metadata(&pb);
            match omd {
                Ok(md) => {
                    let mut ret;
                    if md.is_file() {
                        log::debug!("Got metadata: {:?}", &md);
                        #[cfg(target_family = "unix")]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            if md.permissions().mode() & 0o111 != 0 {
                                ret = Some(Version::OutOfDate(None));
                            } else {
                                continue;
                            }
                        }
                        #[cfg(not(target_family = "unix"))]
                        {
                            ret = Some(Version::OutOfDate(None));
                        }

                        let degraded = if let Ok(o) = process::Command::new(&pb).args(&["--version"]).output() {
                            if let Ok(Some(data)) = stdstr::from_utf8(&o.stdout).map(|d| d.trim().split(' ').last().map(|s|s.to_string())) {
                                if data == version {
                                    ret = Some(Version::UpToDate(version));
                                    "OK".to_string()
                                } else {
                                    let s = format!("unexpected version {}", &data);
                                    ret = Some(Version::OutOfDate(Some(data)));
                                    s
                                }
                            } else {
                                "no readable version information".to_string()
                            }
                        } else {
                            "no version information".to_string()
                        };

                        log::info!("{:?} found ({})", &pb, &degraded);

                        if let Some(Version::OutOfDate(_)) = ret {
                            log::warn!("Got {} from MFEK{}. Your experience may be degraded. Please either update MFEK{1} or this program so that the version of MFEK{1} it expects matches. (Expected MFEK{1} {}.)", degraded, module, version);
                        }
                        if let Some(s) = ret {
                            return Ok((s, pb));
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }

    log::error!(
        "Module MFEK{} is not available. MFEK is modular software; it will still run but some \
        features will not be available. For the best experience, please install all available \
        MFEK modules into your PATH.",
        module
    );

    Err(())
}
