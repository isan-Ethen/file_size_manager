pub trait ManageFile {
    fn new(entries: Vec<String>, options: u8) -> Self
    where
        Self: Sized;
    fn process(&self) -> std::io::Result<()>;
}
