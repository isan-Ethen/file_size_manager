use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::io::Write;

pub struct Merger {
    entries: Vec<String>,
    options: u8,
    fail_list: Vec<String>,
}

impl Merger {
    fn new(entries: Vec<String>, options: u8) -> Self {
        Self {
            entries,
            options,
            fail_list: Vec::new(),
        }
    }

    pub fn run(entries: Vec<String>, options: u8) -> std::io::Result<()> {
        let merger: Merger = Merger::new(entries, options);

        for dir in merger.entries.iter() {
            merger.merge(dir)?;
        }
        Ok(())
    }

    fn merge(&self, dirname: &String) -> std::io::Result<()> {
        let filename: String = dirname.replace("sp-", "").replace("_", ".");

        let mut new_f: File = File::create(filename)?;
        let mut buffer: Vec<u8> = Vec::new();

        for entry in fs::read_dir(dirname)? {
            let entry: DirEntry = entry?;
            let mut f: File = File::open(entry.path())?;
            let mut temp: Vec<u8> = Vec::new();
            f.read_to_end(&mut temp)?;
            buffer.extend_from_slice(&*temp);
        }

        new_f.write_all(&buffer)?;

        Ok(())
    }
}
