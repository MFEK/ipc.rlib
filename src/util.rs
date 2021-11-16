use std::path::{Path, PathBuf};

pub trait InUfo<P: AsRef<Path>> {
    fn ufo(&self) -> Option<PathBuf>;
}

impl<P: AsRef<Path>> InUfo<P> for P {
    fn ufo(&self) -> Option<PathBuf> {
        fn is_ufo<P: AsRef<Path>>(p: P) -> bool {
            let ufo = p
                .as_ref()
                .file_name()
                .map(|f|f.to_string_lossy().to_lowercase())
                .unwrap_or_else(||"".into());
            ufo.ends_with(".ufo") || ufo.ends_with(".ufo3")
        }
        fn is_glyphs<P: AsRef<Path>>(p: P) -> bool {
            ["glyphs", "glyphs."].iter().any(|g| {
                p.as_ref()
                    .file_name()
                    .map(|f|f.to_str().unwrap())
                    .unwrap_or_else(||"".into())
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
