use std::{fs, io::Write};

pub fn sign() -> Result<(), anyhow::Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("eula.txt")?;

    file.write_all(b"# Signed by pap\neula=true")?;

    Ok(())
}
