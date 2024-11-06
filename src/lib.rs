use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use tar::Archive;

pub(crate) fn uncompress_tgz(path: &PathBuf, dest: &Path) -> Result<(), std::io::Error> {
    // let path = "archive.tar.gz";
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(dest)?;

    Ok(())
}
