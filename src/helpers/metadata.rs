use crate::IPCInfo;

use csv;
use log;

use std::collections::HashMap;
use std::{iter, process, str as stdstr};

static KMDBIN: &str = "MFEKmetadata";

pub fn arbitrary(info: &IPCInfo, keys: &[&str]) -> Result<HashMap<String, String>, ()> {
    match &info.font.as_ref() {
        Some(ref font) => {
            let endless_k = iter::repeat("-k");
            let args = iter::zip(endless_k, keys.iter()).map(|(e,k)|[e,k]).flatten().collect::<Vec<_>>();
            let mut command_c = process::Command::new(KMDBIN);
            command_c.arg(font)
                .args(&["arbitrary"])
                .args(args);
            log::debug!("Args are {:?}", command_c);
            let command_o = command_c.output();
            let command;

            if command_o.is_ok() { command = command_o.unwrap(); } else { log::error!("{:?}", command_o); return Err(()) }

            let data = if let Ok(csvdata) = stdstr::from_utf8(&command.stdout) {
                csvdata
            } else {
                log::error!("Encoding error?");
                return Err(())
            };

            let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(data.as_bytes());

            let csvrows = if let Ok(c) = reader.records().collect::<Result<Vec<_>, _>>() {
                c
            } else {
                return Err(())
            };

            let ncsvrows = csvrows.len();

            if ncsvrows != keys.len() {
                if keys.len() == 0 {
                    log::warn!("Got nothing from MFEKmetadata, font corrupt?");
                } else {
                    log::warn!("Mismatch! Got {} keys, expected {}. Aborting.", ncsvrows, keys.len());
                }
                Err(())
            } else {
                let mut hm: HashMap<String, String> = HashMap::new();
                for (i, line) in csvrows.iter().enumerate() {
                    let mut sline: Vec<String> = line.iter().map(|r|r.to_string()).collect();
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
                Ok((asc_desc["ascender"].parse().unwrap(), asc_desc["descender"].parse().unwrap()))
            } else {
                Err(())
            }
        }
        None => Err(()),
    }
}
