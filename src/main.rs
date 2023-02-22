// Based on with modification: https://www.youtube.com/watch?v=9WOUDlTS-rg

use std::fs::File;
use std::net::Ipv4Addr;

#[derive(Debug)]
enum CustomError {
    IOErr(std::io::Error),
    AddrErr(std::net::AddrParseError),
    StdErr(Box<dyn std::error::Error>),
    SrvErr,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError {:?}", self)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IOErr(error)
    }
}

impl From<std::net::AddrParseError> for CustomError {
    fn from(error: std::net::AddrParseError) -> Self {
        CustomError::AddrErr(error)
    }
}

impl From<Box<dyn std::error::Error>> for CustomError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        CustomError::StdErr(error)
    }
}

fn main() -> Result<(), CustomError> {
    println!("Error handling example!");
    custom_error()?;
    std_via_custom_error()?;
    std_error()?;
    Ok(())
}

fn std_via_custom_error() -> Result<(), CustomError> {
    let _net = File::open("some-non-existant-file.txt")?;
    let _localhost: Ipv4Addr = "::1".parse()?;
    Ok(())
}

fn std_error() -> Result<(), Box<dyn std::error::Error>> {
    let _net = File::open("some-non-existant-file.txt")?;
    let _localhost: Ipv4Addr = "::1".parse()?;
    Ok(())
}

fn custom_error() -> Result<(), CustomError> {
    Err(CustomError::SrvErr)
}
