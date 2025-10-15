# some_default_dirs

A small Rust crate that returns the path to the **Start Menu Programs folder** on Windows.

## Platform support
| function           | Windows                                                                 | macOS | Linux |
|--------------------|-------------------------------------------------------------------------|--------|--------|
| `startmenu_dir()`  | `%APPDATA%\Microsoft\Windows\Start Menu\Programs`                      | Error     | Error     |

## Verwendung

```rust
use some_default_dirs::startmenu_dir;

fn main() {
    match startmenu_dir() {
        Ok(path) => println!("Startmenu Folder: {}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
