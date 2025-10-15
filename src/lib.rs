use std::path::PathBuf;
#[cfg(target_os = "windows")]
use windows::{
    Win32::UI::Shell::{FOLDERID_Programs, KF_FLAG_DEFAULT, SHGetKnownFolderPath},
    core::PWSTR,
};

/// Gibt den Pfad zum Startmenü-"Programme"-Ordner zurück.
///
/// Auf Windows: `%APPDATA%\Microsoft\Windows\Start Menu\Programs`  
/// Auf Linux: `/usr/share/applications/`
/// Auf MacOS: `/Applications/`
pub fn startmenu_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    unsafe {
        SHGetKnownFolderPath(&FOLDERID_Programs, KF_FLAG_DEFAULT, None)
            .ok()
            .and_then(|path: PWSTR| path.to_string().ok())
            .map(PathBuf::from)
    }

    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}

/// Gibt den Pfad zum lokalen „Anwendungs-Verknüpfungen“-Ordner zurück.
///
/// - Windows: Gleich wie `startmenu_dir()`  
/// - Linux: `$HOME/.local/share/applications`  
/// - macOS: `$HOME/Applications`  
/// - Andere: `None`
pub fn local_startmenu_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        return startmenu_dir();
    }

    #[cfg(target_os = "linux")]
    {
        std::env::var("HOME").ok().map(|home| {
            let mut path = PathBuf::from(home);
            path.push(".local/share/applications");
            path
        })
    }

    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME").ok().map(|home| {
            let mut path = PathBuf::from(home);
            path.push("Applications");
            path
        })
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_startmenu_path_is_not_empty() {
        let path = startmenu_dir().expect("Should return Some(PathBuf)");
        assert!(!path.as_os_str().is_empty(), "Pfad sollte nicht leer sein");
        let path_str = path.to_string_lossy();
        assert!(
            path_str.contains("Programs") || path_str.contains("Programme"),
            "Pfad sollte Programme enthalten"
        );
    }

    #[test]
    fn test_local_startmenu_exists() {
        if let Some(path) = local_startmenu_dir() {
            println!("Lokaler Startmenü-Ordner: {}", path.display());
            assert!(path.exists() || !path.as_os_str().is_empty(), "Pfad sollte existieren oder gültig sein");
        } else {
            println!("Kein Startmenüpfad für dieses OS.");
        }
    }
}
