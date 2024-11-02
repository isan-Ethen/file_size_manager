use crate::file_size_manager::RunCommandError;
use std::fs::{self, File};
use std::io::Read;
use std::io::Seek;
use std::io::Write;

// 50 MiB
const MIN_SIZE: u64 = 52428800;

#[allow(unused)]
pub struct Splitter<I>
where
    I: Iterator<Item = String>,
{
    entries: I,
    options: Vec<Opt>,
    fail_list: Vec<(String, RunCommandError)>,
}

impl<I> Splitter<I>
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
        let mut splitter: Splitter<I> = Splitter::new(entries, options);

        while let Some(entry) = splitter.next_entry() {
            if let Err(err) = splitter.split(&entry) {
                match err.kind() {
                    std::io::ErrorKind::Other => splitter.add_failure(
                        entry,
                        RunCommandError::WrongEntry(
                            err.get_ref().expect("no error message").to_string(),
                        ),
                    ),
                    _ => splitter.add_failure(
                        entry,
                        RunCommandError::ProcessFail(
                            err.get_ref().expect("no error message").to_string(),
                        ),
                    ),
                }
            }
        }

        splitter.display_result();
    }

    fn next_entry(&mut self) -> Option<String> {
        self.entries.next()
    }

    fn split(&self, entry: &String) -> std::io::Result<()> {
        let mut f: File = File::open(entry)?;
        let output_dir: String = format!("sep-{}", entry.replace(".", "_"));
        let len: u64 = f.metadata()?.len();

        if len < MIN_SIZE {
            return Ok(());
        }

        fs::create_dir_all(&output_dir)?;

        let mut cnt: i32 = 0;

        let buffer: &mut [u8] = &mut [0; MIN_SIZE as usize - 32];

        while f.stream_position()? < len - MIN_SIZE {
            f.read_exact(buffer)?;
            let mut new_f: File = File::create(format!("{}/{}-{}.sep", output_dir, entry, cnt))?;
            new_f.write_all(buffer)?;
            cnt += 1;
        }

        let mut remain_data: Vec<u8> = Vec::new();
        f.read_to_end(&mut remain_data)?;
        self.write_part(output_dir, entry, cnt, remain_data)
    }

    fn write_part<P: AsRef<[u8]>>(
        &self,
        output_dir: String,
        base_name: &str,
        cnt: i32,
        data: P,
    ) -> std::io::Result<()> {
        let part_filename = format!("{}/{}-{}.sep", output_dir, base_name, cnt);
        let mut part_file = File::create(part_filename)?;
        part_file.write_all(data.as_ref())
    }

    fn display_result(self) {
        if !self.fail_list.is_empty() {
            eprintln!("Entries failed to process:");
            for (entry, err) in self.fail_list {
                eprintln!("{entry}: {err}");
                if let Err(err) = fs::remove_dir(&entry) {
                    match err.kind() {
                        std::io::ErrorKind::NotFound => {}
                        _ => eprintln!("failed to remove {}", entry),
                    }
                }
            }
        }
    }
}
