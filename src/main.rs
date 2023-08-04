use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let personalize_key_result = hkcu.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
        KEY_READ | KEY_WRITE,
    );

    let personalize_key = match personalize_key_result {
     Ok(p) => p, 
     Err(why) => panic!("Failed to open registry key for personalization: {:?}", why)   
    };

    // get the setting, toggle it between 0 and 1
    let uselt: u32 = personalize_key.get_value("AppsUseLightTheme")?;
    let val: u32 = if uselt > 0 { 0u32 } else { 1u32 };
    personalize_key.set_value("AppsUseLightTheme", &val)?;

    Ok(())
}
