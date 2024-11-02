use crate::file_size_manager::RunCommandError;
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

#[allow(unused)]
pub struct Merger<I>
where
    I: Iterator<Item = String>,
{
    entries: I,
    options: u8,
    fail_list: Vec<(String, RunCommandError)>,
}

impl<I> Merger<I>
where
    I: Iterator<Item = String>,
{
    fn new(entries: I, options: u8) -> Self {
        Self {
            entries,
            options,
            fail_list: Vec::new(),
        }
    }

    fn add_failure(&mut self, entry: String, err: RunCommandError) {
        self.fail_list.push((entry, err));
    }

    pub fn run(entries: I, options: u8) {
        let mut merger: Merger<I> = Merger::new(entries, options);

        while let Some(entry) = merger.next_entry() {
            if let Err(err) = merger.merge(&entry) {
                match err.kind() {
                    std::io::ErrorKind::Other => merger.add_failure(
                        entry,
                        RunCommandError::WrongEntry(
                            err.get_ref().expect("not error message").to_string(),
                        ),
                    ),
                    _ => merger.add_failure(
                        entry,
                        RunCommandError::ProcessFail(
                            err.get_ref().expect("not error message").to_string(),
                        ),
                    ),
                }
            }
        }

        merger.display_result();
    }

    fn next_entry(&mut self) -> Option<String> {
        self.entries.next()
    }

    fn merge(&self, entry: &String) -> std::io::Result<()> {
        let file: String = entry.replace("sep-", "").replace("_", ".");

        let mut new_f: File = File::create(file)?;
        let mut buffer: Vec<u8> = Vec::new();
        let mut entries: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(entry)? {
            let entry = entry?;
            entries.push(entry.path());
        }

        entries.sort();

        for entry in entries {
            println!("{:?}", entry);
            buffer.extend_from_slice(&self.read_file_contents(entry)?);
        }

        new_f.write_all(&buffer)
    }

    fn read_file_contents(&self, path: std::path::PathBuf) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(content)
    }

    fn display_result(self) {
        if !self.fail_list.is_empty() {
            eprintln!("Entries failed to process:");
            for (entry, err) in self.fail_list {
                eprintln!("{entry}: {err}");
                if let Err(err) = fs::remove_file(&entry) {
                    match err.kind() {
                        std::io::ErrorKind::NotFound => {}
                        _ => eprintln!("failed to remove {}", entry),
                    }
                }
            }
        }
    }
}
