#![windows_subsystem = "console"]

use std::{
    fs::File,
    io::{read_to_string, BufReader},
    path::{Path, PathBuf},
    process::exit,
};
use serde_derive::Deserialize;

#[derive(Debug)]
#[derive(Deserialize)]
struct InstallConfiguration {
    install_location: PathBuf,
    set_path: bool,
}

fn main()
{
    let resources: &Path = Path::new("./resources");
    let install: PathBuf = resources.join("install.toml");
    let bin: PathBuf = resources.join("bin");

    {// Let's ensure everything is how it should be. If not, exit.
        if !resources.try_exists().unwrap_or(false)
            || !install.try_exists().unwrap_or(false)
            || !bin.try_exists().unwrap_or(false)
        {
            eprintln!(
                "One or more of the required files or directories couldn't be found or accessed!"
            );
            exit(1);
        }

        if !resources.is_dir()
        {
            eprintln!(
                "Expected {} to be a directory!",
                resources.to_string_lossy()
            );
            exit(1);
        }

        if !bin.is_dir()
        {
            eprintln!("Expected {} to be a directory!", bin.to_string_lossy());
            exit(1);
        }



        if !install.is_file()
        {
            eprintln!("Expected {} to be a directory!", install.to_string_lossy());
            exit(1);
        }
    }

    let install_contents = read_to_string(BufReader::new(
        File::open(install.clone())
            .expect(format!("Couldn't open file {}", install.to_string_lossy()).as_str()),
    )).unwrap();

    let config: InstallConfiguration = toml::from_str(&install_contents).unwrap();
    println!("CONFIG WORKED: {:?}", config);
}
