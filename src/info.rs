use log;

use std::env;
use std::path::{Path, PathBuf};

use crate::util::InUfo as _;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct IPCInfo {
    pub parent_module: String,
    pub parent_exe: PathBuf,
    pub font: Option<PathBuf>,
    pub glyph: Option<PathBuf>,
}

impl IPCInfo {
    fn from_fields(parent_module: String, font: Option<PathBuf>, glyph: Option<PathBuf>) -> Self {
        IPCInfo { parent_module, parent_exe: env::current_exe().unwrap(), font, glyph }
    }

    pub fn from_font_dir(parent: String, path: &impl AsRef<Path>) -> Self {
        IPCInfo::from_fields(parent, Some(path.as_ref().to_path_buf()), None)
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
        IPCInfo::from_fields(parent, font, Some(path.as_ref().to_path_buf()))
    }

    pub fn from_glif_path(parent: String, path: &impl AsRef<Path>) -> Self {
        let font = path
            .as_ref()
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .ufo();

        IPCInfo::from_fields(parent, font, Some(path.as_ref().to_path_buf()))
    }

    pub fn new_disconnected() -> Self {
        log::debug!("You probably don't want to be making a disconnected IPC info struct. It's only generally useful for local tests…");
        IPCInfo::from_fields(super::KMDBIN.to_string(), None, None)
    }
}
