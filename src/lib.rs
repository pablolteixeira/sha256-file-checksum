use sha2::{Sha256, Digest};
use std::fs;
use std::error::Error;
use std::io;
use hex;
use std::path::PathBuf;
use std::env;

pub struct Config(pub String);

impl Config {
    pub fn build(arguments: &[String]) -> Result<Self, &'static str> {
        if arguments.len() < 2 {
            return Err("must have the file/image path");
        }
        let dir = env::current_dir().unwrap().into_os_string().into_string().unwrap();

        let path = PathBuf::from(&dir).join(arguments[1].clone())
                           .canonicalize().unwrap().into_os_string().into_string().unwrap();

        Ok(Self(path))
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut file = fs::File::open(&config.0)?;
    let mut hasher = Sha256::new();
    
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    let hex_string = hex::encode(&hash);

    println!("{}  {}", hex_string, config.0);

    Ok(())
}