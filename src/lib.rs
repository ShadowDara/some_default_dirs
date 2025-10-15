use std::path::PathBuf;

#[cfg(target_os = "windows")]
use windows::{
    Win32::UI::Shell::{FOLDERID_Programs, KF_FLAG_DEFAULT, SHGetKnownFolderPath},
    core::PWSTR,
};

/// Returns the path to the Start Menu "Programs" folder.
///
/// On Windows: `%APPDATA%\Microsoft\Windows\Start Menu\Programs`  
/// On Linux: `/usr/share/applications/`  
/// On macOS: `/Applications/`
/// **Others**: `None`
pub fn startmenu_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    unsafe {
        SHGetKnownFolderPath(&FOLDERID_Programs, KF_FLAG_DEFAULT, None)
            .ok()
            .and_then(|path: PWSTR| path.to_string().ok())
            .map(PathBuf::from)
    }

    #[cfg(target_os = "linux")]
    {
        Some(PathBuf::from("/usr/share/applications/"))
    }

    #[cfg(target_os = "macos")]
    {
        Some(PathBuf::from("/Applications/"))
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

/// Returns the path to the user-local application shortcuts folder.
///
/// - Windows: Same as `startmenu_dir()`  
/// - Linux: `$HOME/.local/share/applications`  
/// - macOS: `$HOME/Applications`  
/// - Other: `None`
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
    use std::path::Path;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_startmenu_path_is_not_empty() {
        let path = startmenu_dir().expect("Expected Some(PathBuf)");
        assert!(
            !path.as_os_str().is_empty(),
            "Path should not be empty"
        );
        let path_str = path.to_string_lossy();
        assert!(
            path_str.contains("Programs") || path_str.contains("Programme"),
            "Path should contain 'Programs' or 'Programme'"
        );
    }

    #[test]
    fn test_local_startmenu_exists_or_is_valid() {
        if let Some(path) = local_startmenu_dir() {
            println!("Local start menu folder: {}", path.display());
            assert!(
                path.exists() || !path.as_os_str().is_empty(),
                "Path should exist or be a valid non-empty path"
            );
        } else {
            println!("No start menu path available on this OS.");
        }
    }

    #[cfg(target_os = "windows")]
    mod windows_tests {
        use super::*;

        #[test]
        fn test_startmenu_dir_returns_valid_path() {
            let path = startmenu_dir().expect("Expected Some(PathBuf)");
            assert!(
                !path.as_os_str().is_empty(),
                "Startmenu path should not be empty"
            );
            let path_str = path.to_string_lossy().to_lowercase();
            assert!(
                path_str.contains("programs") || path_str.contains("programme"),
                "Startmenu path should contain 'Programs' or 'Programme'"
            );
            assert!(
                Path::new(&path).exists(),
                "Startmenu path should exist on disk"
            );
        }

        #[test]
        fn test_local_startmenu_dir_equals_startmenu_dir() {
            let global_path = startmenu_dir();
            let local_path = local_startmenu_dir();

            assert_eq!(
                global_path, local_path,
                "On Windows, local_startmenu_dir() should equal startmenu_dir()"
            );
        }
    }

    #[cfg(target_os = "linux")]
    mod linux_tests {
        use super::*;

        #[test]
        fn test_startmenu_dir_returns_usr_share_applications() {
            let path = startmenu_dir().expect("Expected Some(PathBuf)");
            assert_eq!(
                path,
                PathBuf::from("/usr/share/applications/"),
                "Linux startmenu_dir() should return /usr/share/applications/"
            );
            assert!(
                Path::new(&path).exists(),
                "The /usr/share/applications/ directory should exist"
            );
        }

        #[test]
        fn test_local_startmenu_dir_returns_local_applications() {
            let local = local_startmenu_dir().expect("Expected Some(PathBuf)");
            let home = std::env::var("HOME").expect("HOME env var must be set");
            let expected = PathBuf::from(home).join(".local/share/applications");
            assert_eq!(local, expected);
        }

        #[test]
        fn test_local_startmenu_dir_path_exists_or_not() {
            if let Some(local) = local_startmenu_dir() {
                println!("Local startmenu dir: {}", local.display());
                // We don't require this path to always exist (some minimal check)
                assert!(
                    !local.as_os_str().is_empty(),
                    "Local startmenu dir path should not be empty"
                );
            } else {
                panic!("local_startmenu_dir() returned None on Linux unexpectedly");
            }
        }
    }

    #[cfg(target_os = "macos")]
    mod macos_tests {
        use super::*;

        #[test]
        fn test_startmenu_dir_returns_applications_folder() {
            let path = startmenu_dir().expect("Expected Some(PathBuf)");
            assert_eq!(
                path,
                PathBuf::from("/Applications/"),
                "macOS startmenu_dir() should return /Applications/"
            );
            assert!(
                Path::new(&path).exists(),
                "/Applications/ should exist on macOS"
            );
        }

        #[test]
        fn test_local_startmenu_dir_returns_home_applications() {
            let local = local_startmenu_dir().expect("Expected Some(PathBuf)");
            let home = std::env::var("HOME").expect("HOME env var must be set");
            let expected = PathBuf::from(home).join("Applications");
            assert_eq!(local, expected);
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    mod other_os_tests {
        use super::*;

        #[test]
        fn test_startmenu_dir_returns_none() {
            assert!(
                startmenu_dir().is_none(),
                "startmenu_dir() should return None on unsupported OS"
            );
        }

        #[test]
        fn test_local_startmenu_dir_returns_none() {
            assert!(
                local_startmenu_dir().is_none(),
                "local_startmenu_dir() should return None on unsupported OS"
            );
        }
    }
}
