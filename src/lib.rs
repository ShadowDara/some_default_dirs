use std::path::PathBuf;
use windows::{
    Win32::UI::Shell::{FOLDERID_Programs, KF_FLAG_DEFAULT, SHGetKnownFolderPath},
    core::{PWSTR, Result},
};

/// Gibt den Pfad zum Startmenü-"Programme"-Ordner zurück (nur Windows).
pub fn startmenu_dir() -> Result<Option<PathBuf>> {
    #[cfg(target_os = "windows")]
    unsafe {
        let path: PWSTR = SHGetKnownFolderPath(&FOLDERID_Programs, KF_FLAG_DEFAULT, None)?;
        let path_string = path.to_string()?;
        Ok(Some(PathBuf::from(path_string)))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_startmenu_path_is_not_empty() {
        let path_opt = startmenu_dir().expect("Should return Result");
        let path = path_opt.expect("Should return Some(PathBuf)");
        assert!(!path.as_os_str().is_empty(), "Pfad sollte nicht leer sein");
        let path_str = path.to_string_lossy();
        assert!(
            path_str.contains("Programs") || path_str.contains("Programme"),
            "Pfad sollte Programme enthalten"
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_non_windows_returns_none() {
        let result = startmenu_dir();
        assert!(result.is_ok(), "Erwartet ein Ok-Ergebnis");
        assert!(result.unwrap().is_none(), "Auf Nicht-Windows sollte None zurückkommen");
    }
}
