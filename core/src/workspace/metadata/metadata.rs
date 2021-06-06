

pub struct Metadata {
    root_dir: str,
}

impl Metadata {

    pub fn create() -> std::io::Result<()> {
        let bytes = include_bytes!("startloot.sh");
        let script = String::from_utf8_lossy(bytes);
        let script = script.replace("{{ROOT_DIR}}", "abc"); // FIXME replace with root_dir name
        print!("{}", script);
        Ok(())
    }
}