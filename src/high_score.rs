use std::fs::File;
use std::io;
use std::io::{Read, Write};
use crate::constants;

pub fn save_score(score: u64) -> io::Result<()> {
    let mut file = File::create(constants::strings::HIGH_SCORE_PATH)?;
    file.write_all(&score.to_le_bytes())?;
    Ok(())
}

pub fn load_score() -> io::Result<u64> {
    let mut buf = [0_u8; 8];
    let mut file = File::open(constants::strings::HIGH_SCORE_PATH)?;
    file.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}