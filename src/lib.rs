pub mod error;
pub mod command;
pub use crate::error::{KvsError, Result};
use command::Command;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::io::{self, BufWriter, BufReader, Read, Seek, SeekFrom, Write};
use std::fs::{self, File, OpenOptions};
use std::ops::Range;


pub struct KvStore {
    path: PathBuf,
    store: HashMap<String, CommandPos>,
    writer: BufWriterWithPos<File>,
    current_index: u64,
    readers: HashMap<u64, BufReaderWithPos<File>>,
}

impl KvStore {

    pub fn new(path: PathBuf)  -> Result<KvStore> {
        let store = HashMap::new();
        let path = path.into();
        let mut readers = HashMap::new();

        let list = list_log_files(&path)?;
        let current_index = list.last().unwrap_or(&0) + 1;
        let writer = new_log_file(&path, current_index)?;
        
        for &index in &list {
            let reader = BufReaderWithPos::new(File::open(path.join(format!("{}.log", index)))?)?;
            //uncompacted += load(gen, &mut reader, &mut index)?;
            readers.insert(index, reader);
        }

        Ok(
            KvStore {
                store,
                path,
                writer,
                current_index,
                readers,
            }
        )
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        let pos = self.writer.pos;
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        if let Command::Set{key, ..} = cmd {
            self.store.insert(key, (self.current_index, pos..self.writer.pos).into());
        }
        
        println!("store: {:?}", self.store);
        Ok(())
    }

    pub fn rm(&mut self, key: String) -> Result<()> {

        if self.store.contains_key(&key) {
            let cmd = Command::remove(key);
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;

            if let Command::Rm{key} = cmd {
                self.store.remove(&key).expect("key not found");
            }
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
        
    }


}


///
/// Recover log files for the given path in a vector.
/// 
fn list_log_files(path: &PathBuf) -> Result<Vec<u64>> {
    let mut list: Vec<u64> = fs::read_dir(&path)?
        .flat_map(|res| -> Result<_> { 
            Ok(res?.path()) 
        })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()                            // Toma path
                .and_then(OsStr::to_str)                // lo convierte a string
                .map(|s| s.trim_end_matches(".log"))    // quita el .log del nombre
                .map(str::parse::<u64>)                 // convierte nombre a u64
        })
        .flatten()  
        .collect();                                     // Transforma iterator en collection
    list.sort_unstable();
    Ok(list)
}


fn new_log_file(path: &Path, name: u64) -> Result<BufWriterWithPos<File>> {
    let path = path.join(format!("{}.log", name));

    let writer = BufWriterWithPos::new(
        OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)?,
    )?;

    Ok(writer)

}

#[derive(Debug)]
struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            gen,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}


struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos += self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new_file_created() {

    }


}




