use crate::file_size_manager::command::{Key, Opt};
use crate::file_size_manager::util::get_content_hash;
use crate::file_size_manager::RunCommandError;
use std::collections::HashMap;
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
    options: HashMap<Key, Opt>,
    fail_list: Vec<(String, RunCommandError)>,
}

impl<I> Merger<I>
where
    I: Iterator<Item = String>,
{
    fn new(entries: I, options: HashMap<Key, Opt>) -> Self {
        Self {
            entries,
            options,
            fail_list: Vec::new(),
        }
    }

    fn add_failure(&mut self, entry: String, err: RunCommandError) {
        self.fail_list.push((entry, err));
    }

    pub fn run(entries: I, options: HashMap<Key, Opt>) {
        let mut merger: Merger<I> = Merger::new(entries, options);

        while let Some(entry) = merger.next_entry() {
            if let Err(err) = merger.merge(&entry) {
                match err.kind() {
                    std::io::ErrorKind::Other => merger.add_failure(
                        entry,
                        RunCommandError::WrongEntry(
                            err.get_ref().expect("no error message").to_string(),
                        ),
                    ),
                    _ => merger.add_failure(
                        entry,
                        RunCommandError::ProcessFail(
                            err.get_ref().expect("no error message").to_string(),
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
        let file: String = self.output_file(entry);

        let mut new_f: File = File::create(file)?;
        let mut buffer: Vec<u8> = Vec::new();
        let mut entries: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(entry)? {
            let entry = entry?;
            entries.push(entry.path());
        }

        entries.sort();

        for entry in entries {
            let content = &self.get_file_contents(entry)?;
            buffer.extend_from_slice(content);
        }

        new_f.write_all(&buffer)
    }

    fn output_file(&self, entry: &String) -> String {
        match self.options.get(&Key::Output) {
            Some(Opt::Output(filename)) => filename.clone(),
            _ => entry.replace("sep-", "").replace("_", "."),
        }
    }

    fn get_file_contents(&self, path: PathBuf) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(&path)?;
        let mut content = Vec::new();
        let mut stored_hash = [0u8; 32];
        file.read_exact(&mut stored_hash)?;
        file.read_to_end(&mut content)?;
        if Merger::<I>::check_hash(stored_hash, get_content_hash(&content)) {
            Ok(content)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("file content edited: {}", path.display()),
            ))
        }
    }

    fn check_hash(stored_hash: [u8; 32], content_hash: [u8; 32]) -> bool {
        let mut eq: bool = true;
        for i in 0..32 {
            if stored_hash[i] != content_hash[i] {
                println!("wrong!: {}, {}", stored_hash[i], content_hash[i]);
                eq = false;
            }
        }
        eq
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
