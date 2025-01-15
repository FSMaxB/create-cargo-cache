use serde::Deserialize;
use std::{fs::File, io::Read, process::Command};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let lockfile_path = std::env::args().nth(1).context("Missing lockfile path")?;

    let mut lockfile = String::new();
    File::open(&lockfile_path)
        .context("Failed to open lock file")?
        .read_to_string(&mut lockfile)
        .context("Failed to read lock file")?;

    let LockFile { packages } =
        toml::de::from_str(&lockfile).context("Failed to deserialize lockfile")?;

    for Package { name, version } in packages {
        let package_specifier = format!("{name}@{version}");
        println!("Caching {package_specifier}");
        // NOTE: Ignoring the status because it will be a failed status for installing library packages anyways
        let _ = Command::new("cargo")
            .arg("install")
            .arg(&package_specifier)
            .status()
            .context("Failed to run cargo install")?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct LockFile {
    #[serde(rename = "package")]
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    name: String,
    version: String,
}
