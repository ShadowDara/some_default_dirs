# some_default_dirs

A small Rust crate that returns the path to the **Start Menu Programs folder** on Windows.

## Platform support
| function | Windows | macOS | Linux |
|--|--|--|--|
| `startmenu_dir()` | `%APPDATA%\Microsoft\Windows\Start Menu\Programs` | `/Applications/` | `/usr/share/applications/` |
| `local_startmenu_dir()` | `%APPDATA%\Microsoft\Windows\Start Menu\Programs` | `$HOME/Applications` | `$HOME/.local/share/applications`  |
|  |  |  |  |

## Verwendung

```rust
use some_default_dirs::startmenu_dir;

fn main() {
    match startmenu_path::startmenu_dir() {
        Some(path) => println!("Startmenu Folder: {}", path.display()),
        None => println!("Function is not supported on this OS."),
    }
}
```

**Contributing is welcomed !**
