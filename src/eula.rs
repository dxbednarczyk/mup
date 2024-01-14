use std::{fs, io::Write};

pub fn sign() -> Result<(), anyhow::Error> {
    let mut file = fs::OpenOptions::new().write(true).truncate(true).open("eula.txt")?;
    
    file.write_all("# Signed by pap\neula=true".as_bytes())?;

    Ok(())
}