use flate2::read::GzDecoder;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use tar::Archive;

use crate::utils;

pub(crate) fn uncompress_tgz(path: &PathBuf, dest: &Path) -> Result<(), std::io::Error> {
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    for entry in archive.entries()? {
        let _path = String::from(entry?.path()?.to_str().unwrap());
        if !utils::is_safe_path(&_path) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("unsafe path: {_path}"),
            ));
        }
    }

    archive.unpack(dest)?;

    Ok(())
}
