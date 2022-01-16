#[cfg(target_family = "windows")]
use ansi_term;
use colored::Colorize as _;
use figlet_rs::FIGfont;

use chrono::TimeZone;

use std::io::{self, Write as _};

static MFEK: &str = r#"
      ___           ___         ___           ___     
     /\  \         /\__\       /\__\         /|  |    
    |::\  \       /:/ _/_     /:/ _/_       |:|  |    
    |:|:\  \     /:/ /\__\   /:/ /\__\      |:|  |    
  __|:|\:\  \   /:/ /:/  /  /:/ /:/ _/_   __|:|  |    
 /::::|_\:\__\ /:/_/:/  /  /:/_/:/ /\__\ /\ |:|__|____
 \:\~~\  \/__/ \:\/:/  /   \:\/:/ /:/  / \:\/:::::/__/
  \:\  \        \::/__/     \::/_/:/  /   \::/~~/~    
   \:\  \        \:\  \      \:\/:/  /     \:\~~\     
    \:\__\        \:\__\      \::/  /       \:\__\    
     \/__/         \/__/       \/__/         \/__/    "#;

pub fn header(module: &str) -> Vec<u8> {
    let buf: String = MFEK.to_string();
    let lines: Vec<_> = buf.lines().rev().collect();
    let mfek_len = lines.len();

    let slant = FIGfont::from_content(include_str!("../resources/slant.flf")).unwrap();
    let mut module_slant = slant.convert(module).unwrap().to_string();
    module_slant = "\n".repeat(mfek_len) + &module_slant;

    (module_slant.lines().rev().collect::<Vec<&str>>().into_iter().zip(lines))
        .rev()
        .map(|(a, b)| b.bold().to_string() + &a.blue().bold().to_string())
        .chain([String::new()])
        .chain([String::new()])
        .collect::<Vec<String>>()
        .join("\n")
        .as_bytes()
        .to_owned()
}

fn header_compiled(compiled: i64) -> String {
    let offset = chrono::Local::now();
    let now: chrono::DateTime<chrono::Local> = offset.timezone().timestamp(compiled, 0);
    let now_fmt = "%Y年%m月%d日(%a)　%H時%M分%S秒(%P)　協定世界時%z";
    let date = chrono_locale::LocaleDate::formatl(&now, &now_fmt, "ja-JP").to_string();
    date.chars()
        .into_iter()
        .map(|c| {
            if c.len_utf8() > 1 {
                c.to_string().green()
            } else {
                c.to_string().normal()
            }
        })
        .map(|cs| cs.to_string())
        .collect()
}

// for graphical applications
pub fn elaborate_display(module: &str, version: &str, compiled: Option<i64>) {
    if let Ok(_) = std::env::var("MFEK_SUPPRESS_HEADER") {
        return;
    }
    display(module);
    #[cfg(not(feature = "reproducible-build"))]
    let cdate = if let Some(compiled) = compiled {
        format!(", compiled @ {}.", &header_compiled(compiled))
    } else {
        format!(".")
    };
    #[cfg(feature = "reproducible-build")]
    let cdate = format!(".");
    let version = match option_env!("MFEK_REL_CODENAME") {
        Some(codename) => format!(" {} (“{}”)", version, codename),
        None => format!(" {}", version),
    };
    let line = format!("This is MFEK{}{}{}\n", module, version, cdate);
    if atty::is(atty::Stream::Stderr) {
        if let Err(_e) = io::stderr().write(line.as_bytes()) {}
    }
}

pub fn display(module: &str) {
    #[cfg(target_family = "windows")]
    if let Err(e) = ansi_term::enable_ansi_support() {
        log::warn!("Failed to enable ANSI term on Windows! error: {:?}", e);
    }

    if let Ok(_) = std::env::var("MFEK_SUPPRESS_HEADER") {
        return;
    }
    if atty::is(atty::Stream::Stderr) {
        if let Err(e) = io::stderr().write(&header(module)) {
            log::error!("Failed to write MFEK ASCII art header?? error: {:?}", e);
        }
    }
}
