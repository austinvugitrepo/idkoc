use clap::Parser; //clap
use std::path::PathBuf; //for file extension
//mod ext; bringing ext.rs

#[derive(Parser, Debug)] // derive works with structs, enums, etc.
#[command(version, long_version = "v0.1.1\nLicense:CC0 1.0 Universal\ncreated by: Austin Vu", about = "idkoc, the best CLI image formatter", long_about = "idkoc (I dont know or care) is a modern CLI image formatter\nwritten in Rust designed to be an all in one image tool")]

struct CliArgs {

    /// webp image format
    #[arg(short, long, value_parser)]
    webp: Option<PathBuf>,
                           //my optional argument
    /// png image format
    #[arg(short, long, value_parser)]
    png: Option<PathBuf>,

}

fn main() {
    let argument = CliArgs::parse(); //argument parser
           // Some means like it exists  or whatever
    if let Some(a) = argument.webp.as_ref().and_then(|p| p.extension()) { //access option and do nested function and unnest it
      if a.to_str() == Some("webp") { //have to convert the fresh <&OsStr> to &str
          println!("This is a .webp file.");
      } else {
          println!("This is NOT a .webp file.");
      }
    }
     if let Some(b) = argument.png.as_ref().and_then(|p| p.extension()) { //access option and do nested function and unnest it
      if b.to_str() == Some("png") { //have to convert the fresh <&OsStr> to &str
          println!("This is a .png file.");
      } else {
          println!("This is NOT a .png file.");
      }
    }


  }
