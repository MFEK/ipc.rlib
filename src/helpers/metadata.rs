use crate::IPCInfo;

use csv;
use glifparser::{Guideline, PointData, IntegerOrFloat::Float};
use log;

use std::collections::HashMap;
use std::{iter, process, str as stdstr};

static KMDBIN: &str = "MFEKmetadata";

pub fn arbitrary(info: &IPCInfo, keys: &[&str]) -> Result<HashMap<String, String>, ()> {
    log::debug!("Getting arbitrary keys: {:?}", keys);
    match &info.font.as_ref() {
        Some(ref font) => {
            let mut endless_k = iter::repeat("-k");
            let args = keys.iter()
                .map(|k| [endless_k.next().unwrap(), k])
                .flatten()
                .collect::<Vec<_>>();
            let mut command_c = process::Command::new(KMDBIN);
            command_c.arg(font).args(&["arbitrary"]).args(args);
            log::trace!("Args are {:?}", command_c);
            let command_o = command_c.output();
            let command;

            if command_o.is_ok() {
                command = command_o.unwrap();
            } else {
                log::error!("{:?}", command_o);
                return Err(());
            }

            let data = if let Ok(csvdata) = stdstr::from_utf8(&command.stdout) {
                csvdata
            } else {
                log::error!("Encoding error?");
                return Err(());
            };

            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(data.as_bytes());

            let csvrows = if let Ok(c) = reader.records().collect::<Result<Vec<_>, _>>() {
                c
            } else {
                return Err(());
            };

            let ncsvrows = csvrows.len();

            if ncsvrows != keys.len() {
                if keys.len() == 0 {
                    log::warn!("Got nothing from MFEKmetadata, font corrupt?");
                } else {
                    log::warn!(
                        "Mismatch! Got {} keys, expected {}. Aborting.",
                        ncsvrows,
                        keys.len()
                    );
                }
                Err(())
            } else {
                let mut hm: HashMap<String, String> = HashMap::new();
                for (i, line) in csvrows.iter().enumerate() {
                    let mut sline: Vec<String> = line.iter().map(|r| r.to_string()).collect();
                    let s = sline.pop().unwrap();
                    hm.insert(keys[i].to_string(), s);
                }
                Ok(hm)
            }
        }
        None => Err(()),
    }
}

pub fn ascender_descender(info: &IPCInfo) -> Result<(f32, f32), ()> {
    match &info.font.as_ref() {
        Some(_) => {
            if let Ok(asc_desc) = arbitrary(info, &["ascender", "descender"]) {
                Ok((
                    asc_desc["ascender"].parse().unwrap(),
                    asc_desc["descender"].parse().unwrap(),
                ))
            } else {
                Err(())
            }
        }
        None => Err(()),
    }
}

pub fn guidelines<PD: PointData>(info: &IPCInfo) -> Result<Vec<Guideline<PD>>, ()> {
    log::debug!("Getting arbitrary keys: {:?}", &["guidelines"]);
    let font = match &info.font {
        Some(font) => {
            font
        },
        None => return Err(())
    };

    let command = process::Command::new(KMDBIN)
        .arg(font)
        .args(&["arbitrary", "-k", "guidelines"])
        .output();

    let mut guidelines = vec![];
    let res = command.map(|output| {
        let lines_vec = stdstr::from_utf8(&output.stdout).unwrap();
        let line: Vec<std::collections::BTreeMap<&str, serde_json::Value>> = if let Some(line) = lines_vec.lines().next() {
            log::trace!("{}", &line);
            if let Ok(v) = serde_json::from_str(line) {
                Ok(v)
            } else {
                Err(())
            }
        } else {
            Err(())
        }?;

        let mut unnamed_i = 0;
        for guideline in line.iter() {
            let (x_o, y_o, angle_o) = (guideline.get("x"), guideline.get("y"), guideline.get("angle"));
            let (name_o, color_o, identifier_o) = (guideline.get("name"), guideline.get("color"), guideline.get("identifier"));
            let mut glifguideline = if let (Some(x_v), Some(y_v), Some(angle_v)) = (x_o, y_o, angle_o) {
                if let (Some(x), Some(y), Some(angle)) = (x_v.as_f64(), y_v.as_f64(), angle_v.as_f64()) {
                    Guideline::from_x_y_angle(x as f32, y as f32, Float(angle as f32))
                } else {
                    continue
                }
            } else { continue };
            if let Some(Some(name)) = name_o.map(|o|o.as_str()) {
                glifguideline = glifguideline.name(name);
            } else {
                unnamed_i += 1;
                glifguideline = glifguideline.name(format!("Unnamed {}", unnamed_i));
            }
            if let Some(Some(identifier)) = identifier_o.map(|o|o.as_str()) {
                glifguideline = glifguideline.identifier(identifier);
            }
            if let Some(Some(color)) = color_o.map(|o|o.as_array()) {
                if let (Some(r), Some(g), Some(b), Some(a)) = (color.get(0), color.get(1), color.get(2), color.get(3)) {
                    if let (Some(r), Some(g), Some(b), Some(a)) = (r.as_f64(), g.as_f64(), b.as_f64(), a.as_f64()) {
                        glifguideline = glifguideline.color([r as f32, g as f32, b as f32, a as f32]);
                    }
                }
            }
            log::trace!("Adding UFO guideline: {:?}", &glifguideline);
            guidelines.push(glifguideline);
            log::trace!("Guideline JSON was {:?}", &guideline);
        }
        Ok(())
    }).unwrap_or(Err(()));

    if let Ok(_) = res {
        Ok(guidelines)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::{self, test};
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
}
