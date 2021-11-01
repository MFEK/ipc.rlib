#![feature(iter_zip, option_result_unwrap_unchecked)]

#[macro_use]
extern crate log;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use std::{env, fs};

pub static KMDBIN: &str = "ipc.rlib";

pub mod helpers;

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

pub trait InUfo<P: AsRef<Path>> {
    fn ufo(&self) -> Option<PathBuf>;
}

impl<P: AsRef<Path>> InUfo<P> for P {
    fn ufo(&self) -> Option<PathBuf> {
        fn is_ufo<P: AsRef<Path>>(p: P) -> bool {
            let ufo = p
                .as_ref()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_lowercase();
            ufo.ends_with(".ufo") || ufo.ends_with(".ufo3")
        }
        fn is_glyphs<P: AsRef<Path>>(p: P) -> bool {
            ["glyphs", "glyphs."].iter().any(|g| {
                p.as_ref()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(g)
            })
        }
        if is_ufo(self) {
            return Some(self.as_ref().to_path_buf())
        }
        let parent = match self.as_ref().parent() {
            Some(p) => p,
            None => { return None },
        };
        if is_ufo(parent) {
            Some(parent.to_path_buf())
        } else if is_glyphs(parent) {
            parent.ufo()
        } else {
            None
        }
    }
}

impl IPCInfo {
    pub fn from_font_dir(parent: String, path: &impl AsRef<Path>) -> Self {
        IPCInfo {
            parent_module: parent,
            font: Some(path.as_ref().to_path_buf()),
            glyph: None,
        }
    }

    pub fn from_fontinfo_path(parent: String, path: &impl AsRef<Path>) -> Self {
        let font = match path.as_ref().canonicalize().unwrap().parent() {
            None => None,
            Some(p) => {
                if path.as_ref().file_name().unwrap() == "fontinfo.plist" {
                    p.ufo()
                } else {
                    None
                }
            }
        };
        IPCInfo {
            parent_module: parent,
            font: font,
            glyph: Some(path.as_ref().to_path_buf()),
        }
    }

    pub fn from_glif_path(parent: String, path: &impl AsRef<Path>) -> Self {
        let font = path
            .as_ref()
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .ufo();

        IPCInfo {
            parent_module: parent,
            font: font,
            glyph: Some(path.as_ref().to_path_buf()),
        }
    }

    pub fn new_disconnected() -> Self {
        debug!("You probably don't want to be making a disconnected IPC info struct. It's only generally useful for local tests…");
        IPCInfo {
            parent_module: KMDBIN.to_string(),
            font: None,
            glyph: None,
        }
    }
}

pub fn module_name(module: &str) -> Vec<String> {
    #[cfg(target_family = "windows")]
    let module = vec![
        format!("MFEK{}.exe", module),
        format!("mfek-{}.exe", module),
    ];
    #[cfg(not(target_family = "windows"))]
    let module = vec![format!("MFEK{}", module), format!("mfek-{}", module)];

    module
}

pub fn module_available(module: &str) -> (Available, String) {
    let mut ret = Available::No;
    let modules = module_name(module);
    for mn in modules.iter() {
        match env::var_os("PATH") {
            Some(paths) => {
                for path in env::split_paths(&paths) {
                    let pb: PathBuf = [path.as_os_str(), &OsString::from(mn.clone())]
                        .iter()
                        .collect();
                    debug!("Checking {:?} for {:?}", &pb, &mn);
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
    error!(
        "Module MFEK{} is not available. MFEK is modular software; it will still run but some \
        features will not be available. For the best experience, please install all available \
        MFEK modules into your PATH.",
        module
    );
    (ret, String::new())
}

#[cfg(test)]
mod tests {
    use super::module_available;
    use std::process;
    use test_env_log::test;

    const KMD: &str = "metadata";
    #[test]
    #[allow(non_snake_case)]
    fn MFEKmetadata_available() {
        let (status, name) = module_available(KMD.into());
        assert!(status.assert());
        assert!(process::Command::new(name).status().is_ok());
    }
}
