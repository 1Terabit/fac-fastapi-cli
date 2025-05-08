use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn append_to_init(init_path: &str, name: &str) {
    let mut content = String::new();
    if Path::new(init_path).exists() {
        fs::File::open(init_path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
    }

    let import_line = format!("from .{} import router as {}_router", name, name);
    if !content.contains(&import_line) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(init_path)
            .unwrap();
        writeln!(file, "{}", import_line).unwrap();
        println!("🔗 Updated '{}'", init_path);
    } else {
        println!("ℹ️  '{}' already registered in '{}'", name, init_path);
    }
}
