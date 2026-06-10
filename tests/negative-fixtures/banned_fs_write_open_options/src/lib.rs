use std::fs::OpenOptions;
use std::io;
use std::path::Path;

pub fn write_file(path: &Path) -> io::Result<()> {
    std::fs::write(path, b"ambient output")?;
    let _file = OpenOptions::new().write(true).open(path)?;
    Ok(())
}
