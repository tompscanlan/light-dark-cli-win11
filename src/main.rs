use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    println!("Reading some system info...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let personalize = hkcu.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", KEY_READ | KEY_WRITE )?;
    let uselt: u32 =personalize.get_value("AppsUseLightTheme")?;
    println!("AppsUseLightTheme is currently = {}", uselt);

    let val: u32 = if uselt > 0 { 0u32} else { 1u32};

    personalize.set_value("AppsUseLightTheme", &val)?;

    let uselt: u32 =personalize.get_value("AppsUseLightTheme")?;
    println!("Setting AppsUseLightTheme = {}", uselt);
    
    Ok(())
}