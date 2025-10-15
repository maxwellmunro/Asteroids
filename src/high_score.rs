use std::fs::File;
use std::io;
use std::io::{Read, Write};

pub fn save_score(path: &str, score: u64) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&score.to_le_bytes())?;
    Ok(())
}

pub fn load_score(path: &str) -> io::Result<u64> {
    let mut buf = [0_u8; 8];
    let mut file = File::open(path)?;
    file.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}