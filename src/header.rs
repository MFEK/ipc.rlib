use colored::Colorize as _;
use figlet_rs::FIGfont;

pub static MFEK: &str = r#"
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

pub fn header(module: &str) -> String {
    let buf: String = MFEK.to_string();
    let lines: Vec<_> = buf.lines().rev().collect();
    let mfek_len = lines.len();

    let slant = FIGfont::from_content(include_str!("../resources/slant.flf")).unwrap();
    let mut module_slant = slant.convert(module).unwrap().to_string();
    module_slant = "\n".repeat(mfek_len) + &module_slant;

    (module_slant
        .lines()
        .rev()
        .collect::<Vec<&str>>()
        .into_iter()
        .zip(lines))
    .rev()
    .map(|(a, b)| {
        b.bold().to_string() + &a.blue().bold().to_string()
    })
    .chain([String::new()])
    .chain([String::new()])
    .collect::<Vec<String>>()
    .join("\n")
}

pub fn display(module: &str) {
    if atty::is(atty::Stream::Stderr) {
        eprint!("{}", header(module));
    }
}
