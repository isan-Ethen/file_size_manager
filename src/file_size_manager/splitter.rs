use std::fs::{File, Metadata};
use std::io::Read;
use std::io::Seek;
use std::io::Write;

const MIN_SIZE: u64 = 10;

pub struct Splitter {
    entries: Vec<String>,
    options: u8,
    fail_list: Vec<String>,
}

impl Splitter {
    fn new(entries: Vec<String>, options: u8) -> Self {
        Self {
            entries,
            options,
            fail_list: Vec::new(),
        }
    }

    pub fn run(entries: Vec<String>, options: u8) -> std::io::Result<()> {
        let splitter: Splitter = Splitter::new(entries, options);
        for file in splitter.entries.iter() {
            let f: File = File::open(file)?;
            let dirname: String = format!("sep-{}", file.replace(".", "_"));
            splitter.split(f, file, dirname)?;
        }
        Ok(())
    }

    fn split(&self, mut f: File, filename: &String, dirname: String) -> std::io::Result<()> {
        let metadata: Metadata = f.metadata()?;
        let len: u64 = metadata.len();

        if len < MIN_SIZE {
            return Ok(());
        }

        let mut cnt: i32 = 0;

        let buffer: &mut [u8; 10] = &mut [0; MIN_SIZE as usize];

        while f.stream_position()? < len - MIN_SIZE {
            f.read_exact(buffer)?;
            let mut new_f: File = File::create(format!("{}/{}-{}.sep", dirname, filename, cnt))?;
            new_f.write_all(buffer)?;
            cnt += 1;
        }

        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer)?;
        let mut new_f: File = File::create(format!("{}/{}-{}.sep", dirname, filename, cnt))?;
        new_f.write_all(&buffer)?;

        Ok(())
    }
}
