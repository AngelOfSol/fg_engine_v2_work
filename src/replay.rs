use flate2::bufread::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

pub type ReplayWriter<T> = DeflateEncoder<BufWriter<T>>;
pub type ReplayReader<T> = DeflateDecoder<BufReader<T>>;

pub type ReplayWriterFile = ReplayWriter<File>;
pub type ReplayReaderFile = ReplayReader<File>;

pub fn create_new_replay_file(folder: &str) -> std::io::Result<ReplayWriterFile> {
    let mut path = PathBuf::new();
    path.push("replay");
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }
    path.push(folder);
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    let filename = chrono::Local::now().format("%Y-%m-%d %H%M.rep").to_string();

    path.push(filename);

    Ok(DeflateEncoder::new(
        BufWriter::new(File::create(path)?),
        Compression::new(9),
    ))
}

pub fn open_replay_file<P: AsRef<Path>>(path: P) -> std::io::Result<ReplayReaderFile> {
    Ok(DeflateDecoder::new(BufReader::new(File::open(path)?)))
}
