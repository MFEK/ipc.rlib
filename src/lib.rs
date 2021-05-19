#[macro_use]
extern crate log;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use std::{env, fs};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Available {
    Yes,
    /// FIXME: Unimplemented. I should add module version checking.
    Degraded,
    No,
}

impl Available {
    pub fn assert(&self) -> bool {
        *self == Available::Yes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IPCInfo {
    pub parent_module: String,
    pub font: Option<PathBuf>,
    pub glyph: Option<PathBuf>,
}

impl IPCInfo {
    pub fn from_font_dir(parent: String, path: &Path) -> Self {
        IPCInfo {
            parent_module: parent,
            font: Some(path.to_path_buf()),
            glyph: None,
        }
    }

    pub fn from_glif_path(parent: String, path: &Path) -> Self {
        let font = match path.canonicalize().unwrap().parent() {
            None => None,
            Some(p) => {
                if p.file_name().unwrap() == "glyphs" || p.file_name().unwrap().to_str().unwrap().starts_with("glyphs.") {
                    match p.parent() {
                        None => None,
                        Some(pp) => {
                            let ufo = pp.file_name().unwrap().to_string_lossy().to_lowercase();
                            if ufo.ends_with(".ufo") || ufo.ends_with(".ufo3") {
                                Some(pp.to_path_buf())
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            }
        };

        IPCInfo {
            parent_module: parent,
            font: font,
            glyph: Some(path.to_path_buf()),
        }
    }
}

pub fn module_name(module: &str) -> String {
    #[cfg(target_family = "windows")]
    let module = format!("{}.exe", module);

    module.to_string()
}

pub fn module_available(module: &str) -> Available {
    let mut ret = Available::No;
    let module = module_name(module);
    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let pb: PathBuf = [path.as_os_str(), &OsString::from(module.clone())]
                    .iter()
                    .collect();
                debug!("Checking {:?} for {:?}", &pb, &module);
                let omd = fs::metadata(&pb);
                match omd {
                    Ok(md) => {
                        if md.is_file() {
                            debug!("Got metadata: {:?}", &md);
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
                            info!("{:?} found", &pb);

                            return ret;
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        None => {}
    }
    error!(
        "Module {:?} is not available. MFEK is modular software; it will still run but some \
        features will not be available. For the best experience, please install all available \
        MFEK modules into your PATH.",
        module
    );
    ret
}

#[cfg(test)]
mod tests {
    use super::module_available;

    use std::process;

    const KMD: &str = "MFEKmetadata";
    #[test]
    #[allow(non_snake_case)]
    fn MFEKmetadata_available() {
        assert!(module_available(KMD.into()).assert());
        assert!(process::Command::new(KMD).status().is_ok());
    }
}
