#![windows_subsystem = "console"]

use serde_derive::Deserialize;
use std::{
    fs::{create_dir, File, read_dir},
    io::{BufReader, Read},
    path::{Path, PathBuf},
    process::exit,
};

#[derive(Debug, Deserialize)]
struct InstallConfiguration
{
    install_location: PathBuf,
    set_path: bool,
}

fn main()
{
    let resources: &Path = Path::new("./resources");
    let install: PathBuf = resources.join("install.toml");
    let bin: PathBuf = resources.join("bin");

    {
        // Let's ensure everything is how it should be. If not, exit.
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
            eprintln!("Expected {} to be a directory!", install.display());
            exit(1);
        }
    }

    // Get the config file contents as a String.
    let mut install_contents = String::new();
    BufReader::new(
        File::open(install.clone())
            .expect(format!("Couldn't open file {}", install.display()).as_str()),
    )
    .read_to_string(&mut install_contents)
    .unwrap();

    // Parse the config file into a configuration struct
    let config: InstallConfiguration = match toml::from_str(&install_contents)
    {
        Err(_) =>
        {
            eprintln!("Expected a proper config file.");
            exit(2);
        }
        Ok(x) =>
        {
            println!("Successfully parsed {}.", install.display());
            x
        }
    };

    // Try to make the target installation location
    mkdir(config.install_location.clone());

    // Make bin directory
    mkdir(config.install_location.join("bin"));
    

    let bin_contents = match read_dir(bin.clone()) {
        Ok(x) => {
            println!("Read contents of {}", bin.display());
            x
        }
        Err(x) => {
            eprintln!("Couldn't read contents of {}: {}", bin.display(), x);
            exit(3);
        }
    };
    
    for bin in bin_contents 
    {
        match bin {
            Ok(x) => {

            }

            Err(x) => 
            {
                eprintln!();
                exit(5);
            }
        }
    }

}

fn mkdir(path: PathBuf)
{
    match create_dir(path.clone())
    {
        Ok(_) => println!(
            "Created directory: {}",
            path.display()
        ),
        Err(x) =>
        {
            eprintln!("Couldn't Create installation directory {}: {}", path.display(), x);
            exit(3);
        },
    }
}