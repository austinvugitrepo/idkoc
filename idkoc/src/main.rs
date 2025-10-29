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

    /// jpg image format
    #[arg(short, long, value_parser)]
    jpg: Option<PathBuf>,


}

fn main() {

let argument = CliArgs::parse(); //argument parser
                                 //Some means it exists or whatever
if let Some(a) = &argument.webp { // reference shennigans  to see if it exists
    if !a.exists() {  // file checker 
        println!("File does not exist."); 
    } else if let Some(ext) = a.extension().and_then(|x| x.to_str()) { //file extension checker that converts <&OsStr> to &str to compare  
        if ext == "webp" {
            println!("This is a .webp file.");
        } else {
            println!("This is NOT a .webp file.");
        }
    } else { // else case if cant read file extension
        println!("Could NOT read the file extension.");
    }
}
// clone of webp function checker 
if let Some(b) = &argument.png {
    if !b.exists() {
        println!("File does not exist.");
    } else if let Some(ext) = b.extension().and_then(|x| x.to_str()) {
        if ext == "png" {
            println!("This is a .png file.");
        } else {
            println!("This is NOT a .png file.");
        }
    } else {
        println!("Could not read the file extension!");
    }
}

//clone of webp function checker
if let Some(c) = &argument.jpg {
    if !c.exists() {
        println!("File does not exist.");
    } else if let Some(ext) = c.extension().and_then(|x| x.to_str()) {
        if ext == "jpg" {
            println!("This is a .jpg file.");
        } else {
            println!("This is NOT a .jpg file.");
        }
    } else {
        println!("Could not read the file extension!");
    }
}

  }
