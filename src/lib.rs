use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

fn uncompress_tgz(path: &str, dest: &str) -> Result<(), std::io::Error> {
    // let path = "archive.tar.gz";
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(dest)?;

    Ok(())
}
