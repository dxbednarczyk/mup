use std::{fs::{self, File}, io::Write};

pub fn sign() -> Result<(), anyhow::Error> {
    let mut file = if !fs::metadata("eula.txt").is_ok() {
        File::create("eula.txt")?
    } else {
        fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("eula.txt")?
    };

    file.write_all(b"# Signed by pap\neula=true")?;

    Ok(())
}
