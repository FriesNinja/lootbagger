

pub struct Metadata {
    root_dir: str,
}

impl Metadata {

    pub fn create() {
        let bytes = include_bytes!("startloot.sh");
        print!("{}", String::from_utf8_lossy(bytes));
    }
}