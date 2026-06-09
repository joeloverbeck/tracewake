use std::fs::File;
use std::io;
use std::path::Path;

pub fn read_file(path: &Path) -> io::Result<()> {
    let _bytes = std::fs::read(path)?;
    let _text = std::fs::read_to_string(path)?;
    let _file = File::open(path)?;
    Ok(())
}
