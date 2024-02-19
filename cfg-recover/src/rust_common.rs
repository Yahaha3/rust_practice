use serde_json::Value;
use std::fs::{read, File};
use std::io::BufReader;
use std::path::{self, Path};
use std::io;
use std::fs;

pub fn read_json(path: String) -> Result<(), Box<dyn std::error::Error>> {

    let pp = Path::new(&path);
    if pp.is_file() || pp.exists() {
        let file = File::open(&pp)?;
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader)?;
        let abs = value["abs"].as_str().unwrap_or("default");
        println!("{:?}", abs);
    } else {

    }
    Ok(())
}

pub fn copy_file(src: String, dest: String) -> io::Result<()> {
    
    let _result = fs::copy(&src, &dest)?;
    Ok(())
}